use crate::{
    err::ProcessingResult,
    proc::{MatchAction::*, MatchMode::*, Processor},
};

#[inline(always)]
pub fn process_bang(proc: &mut Processor) -> ProcessingResult<()> {
    proc.m(IsSeq(b"<!"), Keep).expect();
    proc.m(ThroughChar(b'>'), Keep).require("bang close")?;
    Ok(())
}
