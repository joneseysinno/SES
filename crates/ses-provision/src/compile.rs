use crate::ast::CompiledProvision;
use crate::error::CompileError;

pub fn compile(_source: &str) -> Result<CompiledProvision, CompileError> {
    Err(CompileError::CompilerNotImplemented)
}
