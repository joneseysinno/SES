//! Engineering domain types: quantities, units, and provision expressions.

pub mod serde_impl;

pub mod error;
pub mod expr;
pub mod ops;
pub mod quantity;
pub mod units;

pub use error::EngineerError;
pub use quantity::Quantity;
pub use ses_core::{
    parse_authored, render, Authored, Dim, Rational, UnitId, UnitSym,
};

/// Backward-compatible alias for the runtime dimension type.
pub type DimensionSignature = Dim;

pub use units::{UnitRegistry, UnitSystem};
