//! Code pipeline: stack → gather → applicability → assemble → execute → emit.

pub mod error;
pub mod facts;
pub mod pipeline;

pub use error::PipelineError;
