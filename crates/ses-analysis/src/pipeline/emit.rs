use crate::error::PipelineError;
use crate::pipeline::PipelineContext;

pub struct Emit;

impl Emit {
    pub fn run(&self, _ctx: &PipelineContext) -> Result<(), PipelineError> {
        Err(PipelineError::NotImplemented("emit"))
    }
}
