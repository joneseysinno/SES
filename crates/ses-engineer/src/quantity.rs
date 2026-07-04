use serde::{Deserialize, Serialize};

use crate::serde_impl;
use ses_core::{Rational, UnitId};

/// Physical value stored in its authored unit — no canonical conversion at write time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quantity {
    #[serde(with = "serde_impl::rational")]
    pub value: Rational,
    #[serde(with = "serde_impl::unit_id")]
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
