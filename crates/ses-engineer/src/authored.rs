//! Authored-text grammar and parser (Vocabulary §1.3).
//!
//! Pure syntax — unit symbols stay unresolved strings until `ses-engineer` evaluates.

use crate::rational::Rational;

mod parse;
mod render;

pub use parse::parse_authored;
pub use render::render;

/// A unit symbol exactly as written, unresolved.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnitSym(pub String);

/// Syntactic AST for engineer-authored quantity text.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Authored {
    /// Simple number with optional unit, e.g. `24 ft`, `0.0025`.
    Quantity {
        /// Parsed numeric value.
        value: Rational,
        /// Unit symbol if present.
        unit: Option<UnitSym>,
    },
    /// Feet-inches form, e.g. `3'-6"`, `3'-6 1/2"`.
    FeetInches {
        /// Whole feet.
        feet: Rational,
        /// Inches component if present.
        inches: Option<Rational>,
    },
    /// Product of measures, e.g. `3 × 8 ft`.
    Product(Vec<Authored>),
}
