use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};
use once_cell::sync::Lazy;

use crate::{
    ast::{NodeData, ScriptOrStyleLang},
    parse::{content::ParsedContent, Code},
};

static END: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .kind(Some(AhoCorasickKind::DFA))
        .build(["</style"])
        .unwrap()
});

pub fn parse_style_content(code: &mut Code) -> ParsedContent {
    let (len, closing_tag_omitted) = match END.find(code.as_slice()) {
        Some(m) => (m.start(), false),
        None => (code.rem(), true),
    };
    ParsedContent {
        closing_tag_omitted,
        children: vec![NodeData::ScriptOrStyleContent {
            code: code.copy_and_shift(len),
            lang: ScriptOrStyleLang::CSS,
        }],
    }
}
