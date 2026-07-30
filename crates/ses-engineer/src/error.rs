use thiserror::Error;

use crate::repr_error::{DimError, RationalError};
use crate::unit::UnitId;

/// Engineering computation errors (Vocabulary §1.3).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum EngineerError {
    #[error("incompatible units: {from:?} cannot convert to {to:?}")]
    IncompatibleUnits { from: UnitId, to: UnitId },

    #[error("unknown unit: {0:?}")]
    UnknownUnit(UnitId),

    #[error("unit registry not initialized")]
    RegistryNotInitialized,

    #[error("dimension mismatch: expected {expected}, found {found}")]
    DimensionMismatch {
        expected: crate::Dim,
        found: crate::Dim,
    },

    #[error("unknown unit symbol: {0}")]
    UnknownSymbol(String),

    #[error("tower value cannot be narrowed to an exact rational")]
    Overflow,

    #[error(transparent)]
    Rational(#[from] RationalError),

    #[error(transparent)]
    Dim(#[from] DimError),
}
