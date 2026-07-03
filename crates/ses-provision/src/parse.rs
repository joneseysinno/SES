use crate::ast::ProvisionBlock;
use crate::error::CompileError;
use crate::lex::Token;

pub fn parse(_tokens: &[Token]) -> Result<ProvisionBlock, CompileError> {
    Err(CompileError::ParserNotImplemented)
}
