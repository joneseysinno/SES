use crate::error::PipelineError;
use crate::pipeline::PipelineContext;

pub struct Stack;

impl Stack {
    pub fn run(&self, _ctx: &PipelineContext) -> Result<(), PipelineError> {
        Err(PipelineError::NotImplemented("stack"))
    }
}
