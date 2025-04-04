#[cfg(feature = "css")]
use std::str::from_utf8;
use std::sync::LazyLock;

use aho_corasick::{AhoCorasickBuilder, AhoCorasickKind, MatchKind};
#[cfg(feature = "css")]
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleAttribute};

use crate::{
    Cfg,
    code_gen::attrs::ATTRS,
    entity::encode::encode_entities,
    pattern::Replacer,
    spec::{script::JAVASCRIPT_MIME_TYPES, tag::ns::Namespace},
    whitespace::{collapse_whitespace, left_trim, remove_all_whitespace, right_trim},
};

// To pass validation, entities MUST end with a semicolon.
fn build_whatwg_double_quoted_replacer() -> Replacer {
    Replacer::new(
        AhoCorasickBuilder::new()
            .kind(Some(AhoCorasickKind::DFA))
            .build([b"\""])
            .unwrap(),
        vec![b"&#34;".to_vec()],
    )
}

// To pass validation, entities MUST end with a semicolon.
fn build_whatwg_single_quoted_replacer() -> Replacer {
    Replacer::new(
        AhoCorasickBuilder::new()
            .kind(Some(AhoCorasickKind::DFA))
            .build([b"'"])
            .unwrap(),
        vec![b"&#39;".to_vec()],
    )
}

// TODO Sync with WHITESPACE definition.
static WS: &[(u8, &[u8])] = &[
    (b'\x09', b"&#9"),
    (b'\x0a', b"&#10"),
    (b'\x0c', b"&#12"),
    (b'\x0d', b"&#13"),
    (b'\x20', b"&#32"),
];

// If spec compliance is required, these characters must also be encoded in an unquoted attr value,
// as well as whitespace, `<`, and `>`.
static WHATWG_UNQUOTED: &[(u8, &[u8])] = &[
    (b'"', b"&#34"),
    (b'\'', b"&#39"),
    (b'=', b"&#61"),
    (b'`', b"&#6"),
];

// To pass validation, entities MUST end with a semicolon.
fn build_whatwg_unquoted_replacer() -> Replacer {
    let mut patterns = Vec::<Vec<u8>>::new();
    let mut replacements = Vec::<Vec<u8>>::new();

    // Replace all whitespace with a numeric entity.
    for &(ws, rep) in WS {
        patterns.push(vec![ws]);
        replacements.push({
            let mut ent = rep.to_vec();
            ent.push(b';');
            ent
        });
    }

    // Replace WHATWG-disallowed characters with a numeric entity
    for &(ws, rep) in WHATWG_UNQUOTED {
        patterns.push(vec![ws]);
        replacements.push({
            let mut ent = rep.to_vec();
            ent.push(b';');
            ent
        });
    }

    // Replace all `<` with `&lt;`.
    patterns.push(b"<".to_vec());
    replacements.push(b"&lt;".to_vec());

    // Replace all `>` with `&gt;`.
    patterns.push(b">".to_vec());
    replacements.push(b"&gt;".to_vec());

    Replacer::new(
        AhoCorasickBuilder::new()
            .kind(Some(AhoCorasickKind::DFA))
            .match_kind(MatchKind::LeftmostLongest)
            .build(patterns)
            .unwrap(),
        replacements,
    )
}

static WHATWG_DOUBLE_QUOTED_REPLACER: LazyLock<Replacer> =
    LazyLock::new(build_whatwg_double_quoted_replacer);
static WHATWG_SINGLE_QUOTED_REPLACER: LazyLock<Replacer> =
    LazyLock::new(build_whatwg_single_quoted_replacer);
static WHATWG_UNQUOTED_REPLACER: LazyLock<Replacer> = LazyLock::new(build_whatwg_unquoted_replacer);

pub struct AttrMinifiedValue {
    quoted: bool,
    prefix: &'static [u8],
    data: Vec<u8>,
    start: usize,
    suffix: &'static [u8],
}

impl AttrMinifiedValue {
    pub fn quoted(&self) -> bool {
        self.quoted
    }

    pub fn len(&self) -> usize {
        self.prefix.len() + (self.data.len() - self.start) + self.suffix.len()
    }

    pub fn out(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.prefix);
        out.extend_from_slice(&self.data[self.start..]);
        out.extend_from_slice(self.suffix);
    }

    #[cfg(test)]
    pub fn str(&self) -> String {
        let mut out = Vec::with_capacity(self.len());
        self.out(&mut out);
        String::from_utf8(out).unwrap()
    }
}

pub fn encode_using_double_quotes(val: &[u8]) -> AttrMinifiedValue {
    AttrMinifiedValue {
        quoted: true,
        prefix: b"\"",
        data: WHATWG_DOUBLE_QUOTED_REPLACER.replace_all(val),
        start: 0,
        suffix: b"\"",
    }
}

pub fn encode_using_single_quotes(val: &[u8]) -> AttrMinifiedValue {
    AttrMinifiedValue {
        quoted: true,
        prefix: b"'",
        data: WHATWG_SINGLE_QUOTED_REPLACER.replace_all(val),
        start: 0,
        suffix: b"'",
    }
}

pub fn encode_unquoted(val: &[u8]) -> AttrMinifiedValue {
    AttrMinifiedValue {
        quoted: false,
        prefix: b"",
        data: WHATWG_UNQUOTED_REPLACER.replace_all(val),
        start: 0,
        suffix: b"",
    }
}

pub enum AttrMinified {
    Redundant,
    NoValue,
    Value(AttrMinifiedValue),
}

pub fn minify_attr(
    cfg: &Cfg,
    ns: Namespace,
    tag: &[u8],
    // True if element is <meta> and has an attribute `name` equal to `viewport`.
    is_meta_viewport: bool,
    name: &[u8],
    mut value_raw: Vec<u8>,
) -> AttrMinified {
    let attr_cfg = ATTRS.get(ns, tag, name);

    let do_not_omit = cfg.keep_input_type_text_attr
        && tag == b"input"
        && name == b"type"
        && value_raw.eq_ignore_ascii_case(b"text");

    let should_collapse = attr_cfg.filter(|attr| attr.collapse).is_some();
    let should_trim = attr_cfg.filter(|attr| attr.trim).is_some();
    let should_lowercase = attr_cfg.filter(|attr| attr.case_insensitive).is_some();
    let is_boolean = attr_cfg.filter(|attr| attr.boolean).is_some();
    // An attribute can have both redundant_if_empty and default_value, which means it has two default values: "" and default_value.
    let redundant_if_empty = attr_cfg.filter(|attr| attr.redundant_if_empty).is_some();
    let default_value = attr_cfg.and_then(|attr| attr.default_value);

    if is_meta_viewport {
        remove_all_whitespace(&mut value_raw);
    } else {
        // Trim before checking is_boolean as the entire attribute could be redundant post-minification.
        if should_trim {
            right_trim(&mut value_raw);
            left_trim(&mut value_raw);
        };
        if should_collapse {
            collapse_whitespace(&mut value_raw);
        };
    };

    #[cfg(feature = "css")]
    if name == b"style" {
        let result = match StyleAttribute::parse(
            from_utf8(&value_raw).expect("`style` attribute value contains non-UTF-8"),
            ParserOptions::default(),
        ) {
            Ok(mut sty) => {
                sty.minify(MinifyOptions::default());
                let popt = PrinterOptions {
                    minify: true,
                    ..Default::default()
                };
                match sty.to_css(popt) {
                    Ok(out) => Some(out.code),
                    // TODO Collect error as warning.
                    Err(_err) => None,
                }
            }
            // TODO Collect error as warning.
            Err(_err) => None,
        };
        if let Some(min) = result {
            value_raw = min.into_bytes();
        };
    }

    // Make lowercase before checking against default value or JAVASCRIPT_MIME_TYPES.
    if should_lowercase {
        value_raw.make_ascii_lowercase();
    };

    if !do_not_omit
        && ((value_raw.is_empty() && redundant_if_empty)
            || default_value.filter(|dv| dv == &value_raw).is_some()
            || (tag == b"script"
                && name == b"type"
                && JAVASCRIPT_MIME_TYPES.contains(value_raw.as_slice())))
    {
        return AttrMinified::Redundant;
    };

    if is_boolean || value_raw.is_empty() {
        return AttrMinified::NoValue;
    };

    let encoded = encode_entities(&value_raw, true);

    // When lengths are equal, prefer double quotes to all and single quotes to unquoted.
    let mut min = encode_using_double_quotes(&encoded);
    let sq = encode_using_single_quotes(&encoded);
    if sq.len() < min.len() {
        min = sq;
    };
    let uq = encode_unquoted(&encoded);
    if uq.len() < min.len() {
        min = uq;
    };
    AttrMinified::Value(min)
}
