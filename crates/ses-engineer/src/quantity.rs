use serde::{Deserialize, Serialize};

use crate::units::UnitId;

/// Exact rational value stored as reduced num/den pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}

impl Rational {
    pub const fn new(num: i64, den: i64) -> Self {
        Self { num, den }
    }

    pub const fn one() -> Self {
        Self { num: 1, den: 1 }
    }
}

/// Physical value stored in its authored unit — no canonical conversion at write time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: Rational,
    pub unit: UnitId,
    /// Verbatim input testimony, e.g. `"24 ft"`, `"3 × 8 ft"`.
    pub authored: String,
}

impl Default for Quantity {
    fn default() -> Self {
        Self {
            value: Rational::one(),
            unit: UnitId(0),
            authored: String::new(),
        }
    }
}

impl Quantity {
    pub fn new(value: Rational, unit: UnitId, authored: impl Into<String>) -> Self {
        Self {
            value,
            unit,
            authored: authored.into(),
        }
    }
}
