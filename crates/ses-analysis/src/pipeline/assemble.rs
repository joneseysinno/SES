use crate::error::PipelineError;
use crate::pipeline::PipelineContext;

pub struct Assemble;

impl Assemble {
    pub fn run(&self, _ctx: &PipelineContext) -> Result<(), PipelineError> {
        Err(PipelineError::NotImplemented("assemble"))
    }
}
