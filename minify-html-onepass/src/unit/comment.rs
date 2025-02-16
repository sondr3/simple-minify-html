use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};
use once_cell::sync::Lazy;

use crate::{
    err::ProcessingResult,
    proc::{MatchAction::*, MatchMode::*, Processor},
};

static COMMENT_END: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .kind(Some(AhoCorasickKind::DFA))
        .build(["-->"])
        .unwrap()
});

#[inline(always)]
pub fn process_comment(proc: &mut Processor) -> ProcessingResult<()> {
    proc.m(IsSeq(b"<!--"), Discard).expect();
    proc.m(ThroughSeq(&COMMENT_END), Discard)
        .require("comment end")?;
    Ok(())
}
