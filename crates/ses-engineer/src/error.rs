use thiserror::Error;

use crate::units::UnitId;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EngineerError {
    #[error("incompatible units: {from:?} cannot convert to {to:?}")]
    IncompatibleUnits { from: UnitId, to: UnitId },

    #[error("unknown unit: {0:?}")]
    UnknownUnit(UnitId),

    #[error("unit registry not initialized")]
    RegistryNotInitialized,

    #[error("dimension mismatch: expected {expected}, found {found}")]
    DimensionMismatch { expected: String, found: String },
}
