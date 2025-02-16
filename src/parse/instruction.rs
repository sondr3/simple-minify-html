use std::sync::LazyLock;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};

use crate::{ast::NodeData, parse::Code};

static INSTRUCTION_END: LazyLock<AhoCorasick> = LazyLock::new(|| {
    AhoCorasickBuilder::new()
        .kind(Some(AhoCorasickKind::DFA))
        .build(["?>"])
        .unwrap()
});

pub fn parse_instruction(code: &mut Code) -> NodeData {
    debug_assert!(code.as_slice().starts_with(b"<?"));
    code.shift(2);
    let (len, matched) = match INSTRUCTION_END.find(code.as_slice()) {
        Some(m) => (m.start(), m.end() - m.start()),
        None => (code.rem(), 0),
    };
    let data = code.copy_and_shift(len);
    // It might be EOF.
    code.shift(matched);
    NodeData::Instruction {
        code: data,
        ended: matched > 0,
    }
}
