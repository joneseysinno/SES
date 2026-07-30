//! Provision DSL: lex, parse, compile → ses-engineer expression AST.

pub mod ast;
pub mod compile;
pub mod error;
pub mod lex;
pub mod parse;

pub use ast::{CompiledProvision, ModificationMode, ProvisionBlock, ProvisionKind};
pub use compile::compile;
pub use error::CompileError;
