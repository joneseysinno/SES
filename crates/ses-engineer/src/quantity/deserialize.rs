use serde::Deserializer;

use crate::rational::Rational;
use crate::unit::UnitId;

use super::Quantity;

impl<'de> serde::Deserialize<'de> for Quantity {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct QuantityRep {
            value: Rational,
            unit: UnitId,
            authored: String,
        }
        let rep = QuantityRep::deserialize(deserializer)?;
        Ok(Quantity::new(rep.value, rep.unit, rep.authored))
    }
}
