use serde::Serializer;

use super::Quantity;

impl serde::Serialize for Quantity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Quantity", 3)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("unit", &self.unit)?;
        state.serialize_field("authored", &self.authored)?;
        state.end()
    }
}
