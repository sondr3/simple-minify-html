use memchr::memchr;

use crate::{ast::NodeData, code_gen::codepoints::WHITESPACE, parse::Code};

pub fn parse_doctype(code: &mut Code) -> NodeData {
    debug_assert!(code.as_slice()[..9].eq_ignore_ascii_case(b"<!doctype"));
    code.shift(9);
    code.shift_while_in_lookup(WHITESPACE);
    code.shift_if_next_seq_case_insensitive(b"html");
    code.shift_while_in_lookup(WHITESPACE);
    let (len, matched) = match memchr(b'>', code.as_slice()) {
        Some(m) => (m, 1),
        None => (code.rem(), 0),
    };
    let _ = code.copy_and_shift(len);
    // It might be EOF.
    code.shift(matched);
    NodeData::Doctype { ended: matched > 0 }
}
