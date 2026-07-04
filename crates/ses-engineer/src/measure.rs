use crate::tower::TowerValue;
use crate::unit::UnitId;

mod ephemeral;

/// Ephemeral derived value; no authored testimony (Vocabulary §1.3).
#[derive(Clone, PartialEq)]
pub struct Measure {
    pub value: TowerValue,
    pub unit: UnitId,
}

impl core::fmt::Debug for Measure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Measure")
            .field("value", &self.value)
            .field("unit", &self.unit)
            .finish()
    }
}
