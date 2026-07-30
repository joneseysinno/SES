use crate::error::PipelineError;
use crate::pipeline::PipelineContext;

pub struct Applicability;

impl Applicability {
    pub fn run(&self, _ctx: &PipelineContext) -> Result<(), PipelineError> {
        Err(PipelineError::NotImplemented("applicability"))
    }
}
