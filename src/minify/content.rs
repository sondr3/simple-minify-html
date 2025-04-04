use std::sync::LazyLock;

use aho_corasick::{AhoCorasickBuilder, AhoCorasickKind};

use super::rcdata::minify_rcdata;
use crate::{
    ast::{NodeData, ScriptOrStyleLang},
    cfg::Cfg,
    entity::encode::encode_entities,
    minify::{
        bang::minify_bang, comment::minify_comment, css::minify_css, doctype::minify_doctype,
        element::minify_element, instruction::minify_instruction, js::minify_js,
    },
    pattern::Replacer,
    spec::tag::{
        ns::Namespace,
        whitespace::{WhitespaceMinification, get_whitespace_minification_for_tag},
    },
    whitespace::{collapse_whitespace, is_all_whitespace, left_trim, right_trim},
};

fn build_whatwg_chevron_replacer() -> Replacer {
    Replacer::new(
        AhoCorasickBuilder::new()
            .kind(Some(AhoCorasickKind::DFA))
            .build(["<"])
            .unwrap(),
        vec!["&lt;".into()],
    )
}

static WHATWG_CHEVRON_REPLACER: LazyLock<Replacer> = LazyLock::new(build_whatwg_chevron_replacer);

pub fn minify_content(
    cfg: &Cfg,
    out: &mut Vec<u8>,
    ns: Namespace,
    descendant_of_pre: bool,
    // Use empty slice if none.
    parent: &[u8],
    mut nodes: Vec<NodeData>,
) {
    let &WhitespaceMinification {
        collapse,
        destroy_whole,
        trim,
    } = get_whitespace_minification_for_tag(ns, parent, descendant_of_pre);

    // TODO Document or fix: even though bangs/comments/etc. don't affect layout, we don't collapse/destroy-whole/trim combined text nodes across bangs/comments/etc., as that's too complex and is ambiguous about which nodes should whitespace be deleted from.
    let mut found_first_text_or_elem = false;
    let mut index_of_last_nonempty_text_or_elem: isize = -1;
    let mut index_of_last_text_or_elem: isize = -1;
    for i in 0..nodes.len() {
        let (previous_nodes, next_nodes) = nodes.split_at_mut(i);
        let n = &mut next_nodes[0];
        match n {
            NodeData::Element { name, .. } => {
                if index_of_last_nonempty_text_or_elem > -1 {
                    if let NodeData::Element {
                        next_sibling_element_name,
                        ..
                    } = &mut previous_nodes[index_of_last_nonempty_text_or_elem as usize]
                    {
                        debug_assert!(next_sibling_element_name.is_empty());
                        next_sibling_element_name.extend_from_slice(name);
                    };
                };
                found_first_text_or_elem = true;
                index_of_last_nonempty_text_or_elem = i as isize;
                index_of_last_text_or_elem = i as isize;
            }
            NodeData::Text { value } => {
                if !found_first_text_or_elem {
                    // This is the first element or text node, and it's a text node.
                    found_first_text_or_elem = true;
                    if trim {
                        left_trim(value);
                    };
                };
                // Our parser is guaranteed to output contiguous text as a single node,
                // so the adjacent nodes to a text node (not counting comments/bangs/etc.) should be elements.
                // TODO debug_assert this and add tests.
                if destroy_whole && is_all_whitespace(value) {
                    value.clear();
                } else if collapse {
                    collapse_whitespace(value);
                };
                // Set AFTER processing.
                index_of_last_text_or_elem = i as isize;
                if !value.is_empty() {
                    index_of_last_nonempty_text_or_elem = i as isize;
                };
            }
            _ => {}
        };
    }
    if trim && index_of_last_text_or_elem > -1 {
        if let NodeData::Text { value } =
            nodes.get_mut(index_of_last_text_or_elem as usize).unwrap()
        {
            right_trim(value);
        };
    }

    for (i, c) in nodes.into_iter().enumerate() {
        match c {
            NodeData::Bang { code, ended } => minify_bang(cfg, out, &code, ended),
            NodeData::Comment { code, ended } => minify_comment(cfg, out, &code, ended),
            NodeData::Doctype { legacy, ended } => minify_doctype(out, &legacy, ended),
            NodeData::Element {
                attributes,
                children,
                closing_tag,
                name,
                namespace: child_ns,
                next_sibling_element_name,
            } => minify_element(
                cfg,
                out,
                descendant_of_pre,
                child_ns,
                parent,
                &next_sibling_element_name,
                (i as isize) == index_of_last_nonempty_text_or_elem,
                &name,
                attributes,
                closing_tag,
                children,
            ),
            NodeData::Instruction { code, ended } => minify_instruction(cfg, out, &code, ended),
            NodeData::RcdataContent { typ, text } => minify_rcdata(out, typ, &text),
            NodeData::ScriptOrStyleContent { code, lang: _ } if code.is_empty() => {}
            NodeData::ScriptOrStyleContent { code, lang } => match lang {
                ScriptOrStyleLang::CSS => minify_css(out, &code),
                ScriptOrStyleLang::Data => out.extend_from_slice(&code),
                ScriptOrStyleLang::JS | ScriptOrStyleLang::JSModule => minify_js(out, &code),
            },
            NodeData::Text { value } => {
                let min = encode_entities(&value, false);
                let min = WHATWG_CHEVRON_REPLACER.replace_all(&min);
                out.extend_from_slice(&min);
            }
        };
    }
}
