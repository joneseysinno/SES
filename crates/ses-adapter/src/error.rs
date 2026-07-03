use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("codec error: {0}")]
    Codec(String),

    #[error("engine error: {0}")]
    Engine(#[from] infinite_db::EngineError),

    #[error("schema version mismatch: expected {expected}, found {found}")]
    SchemaVersionMismatch { expected: u8, found: u8 },

    #[error("unknown payload type")]
    UnknownPayload,
}
