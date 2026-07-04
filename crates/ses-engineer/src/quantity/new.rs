use crate::rational::Rational;
use crate::unit::UnitId;

use super::Quantity;

impl Quantity {
    /// Construct an authored quantity with verbatim origin (Vocabulary §1.1).
    pub fn new(value: Rational, unit: UnitId, authored: impl Into<String>) -> Self {
        Self {
            value,
            unit,
            authored: authored.into(),
        }
    }
}
