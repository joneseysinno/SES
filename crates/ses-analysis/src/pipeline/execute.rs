use crate::error::PipelineError;
use crate::pipeline::PipelineContext;

pub struct Execute;

impl Execute {
    pub fn run(&self, _ctx: &PipelineContext) -> Result<(), PipelineError> {
        Err(PipelineError::NotImplemented("execute"))
    }
}
