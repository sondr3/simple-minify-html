use std::sync::LazyLock;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};

pub static TAG_TEXTAREA_END: LazyLock<AhoCorasick> = LazyLock::new(|| {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .kind(Some(AhoCorasickKind::DFA))
        .build(["</textarea"])
        .unwrap()
});
pub static TAG_TITLE_END: LazyLock<AhoCorasick> = LazyLock::new(|| {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .kind(Some(AhoCorasickKind::DFA))
        .build(["</title"])
        .unwrap()
});
