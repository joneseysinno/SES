use crate::error::CompileError;

/// Token placeholder for the provision lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    String(String),
    Number(String),
}

pub fn lex(_source: &str) -> Result<Vec<Token>, CompileError> {
    Err(CompileError::LexerNotImplemented)
}
