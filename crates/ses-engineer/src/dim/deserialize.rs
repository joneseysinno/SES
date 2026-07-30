use serde::de::{Deserialize, Deserializer};

use super::Dim;

#[derive(serde::Deserialize)]
struct DimRep {
    force: i8,
    length: i8,
    time: i8,
    temperature: i8,
}

impl<'de> Deserialize<'de> for Dim {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let rep = DimRep::deserialize(deserializer)?;
        Ok(Dim::new(rep.force, rep.length, rep.time, rep.temperature))
    }
}
