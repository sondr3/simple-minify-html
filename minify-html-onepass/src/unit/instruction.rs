use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};
use once_cell::sync::Lazy;

use crate::{
    err::ProcessingResult,
    proc::{MatchAction::*, MatchMode::*, Processor},
};

static INSTRUCTION_END: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .kind(Some(AhoCorasickKind::DFA))
        .build(["?>"])
        .unwrap()
});

#[inline(always)]
pub fn process_instruction(proc: &mut Processor) -> ProcessingResult<()> {
    proc.m(IsSeq(b"<?"), Keep).expect();
    proc.m(ThroughSeq(&INSTRUCTION_END), Keep)
        .require("instruction end")?;
    Ok(())
}
