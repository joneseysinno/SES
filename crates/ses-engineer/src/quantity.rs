use crate::rational::Rational;
use crate::unit::UnitId;

/// Physical value stored in its authored unit — no canonical conversion at write time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantity {
    pub value: Rational,
    pub unit: UnitId,
    /// Verbatim input testimony, e.g. `"24 ft"`, `"3 × 8 ft"`.
    pub authored: String,
}

mod deserialize;
mod from_authored;
mod new;
mod serialize;

pub use from_authored::from_authored;
