pub mod applicability;
pub mod assemble;
pub mod emit;
pub mod execute;
pub mod gather;
pub mod stack;

pub use applicability::Applicability;
pub use assemble::Assemble;
pub use emit::Emit;
pub use execute::Execute;
pub use gather::Gather;
pub use stack::Stack;

use crate::error::PipelineError;

/// Placeholder input bundle for pipeline stages.
#[derive(Debug, Clone, Default)]
pub struct PipelineContext;

/// Run all six pipeline stages in order (skeleton — each stage returns NotImplemented).
pub fn run_all(_ctx: &PipelineContext) -> Result<(), PipelineError> {
    Stack.run(_ctx)?;
    Gather.run(_ctx)?;
    Applicability.run(_ctx)?;
    Assemble.run(_ctx)?;
    Execute.run(_ctx)?;
    Emit.run(_ctx)?;
    Ok(())
}
