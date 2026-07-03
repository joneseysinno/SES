use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error("pipeline stage not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("missing fact: {0}")]
    MissingFact(String),

    #[error("provision data error: {0}")]
    ProvisionData(String),
}
