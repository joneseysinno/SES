use serde::Deserializer;

use super::UnitId;

impl<'de> serde::Deserialize<'de> for UnitId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(UnitId(u32::deserialize(deserializer)?))
    }
}
