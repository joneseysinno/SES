use serde::Serialize;
use serde::ser::Serializer;

use super::Dim;

#[derive(serde::Serialize)]
struct DimRep {
    force: i8,
    length: i8,
    time: i8,
    temperature: i8,
}

impl Serialize for Dim {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        DimRep {
            force: self.force,
            length: self.length,
            time: self.time,
            temperature: self.temp,
        }
        .serialize(serializer)
    }
}
