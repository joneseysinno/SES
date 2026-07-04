use serde::Serializer;

use super::UnitId;

impl serde::Serialize for UnitId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
