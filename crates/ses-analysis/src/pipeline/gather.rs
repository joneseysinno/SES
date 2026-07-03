use crate::error::PipelineError;
use crate::pipeline::PipelineContext;

pub struct Gather;

impl Gather {
    pub fn run(&self, _ctx: &PipelineContext) -> Result<(), PipelineError> {
        Err(PipelineError::NotImplemented("gather"))
    }
}
