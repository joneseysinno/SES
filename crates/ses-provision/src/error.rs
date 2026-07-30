use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CompileError {
    #[error("lexer not implemented")]
    LexerNotImplemented,

    #[error("parser not implemented")]
    ParserNotImplemented,

    #[error("compiler not implemented")]
    CompilerNotImplemented,

    #[error("dimension mismatch: {0}")]
    DimensionMismatch(String),

    #[error("tower level mismatch: declared {declared:?}, inferred {inferred:?}")]
    TowerLevelMismatch {
        declared: ses_engineer::expr::TowerLevel,
        inferred: ses_engineer::expr::TowerLevel,
    },
}
