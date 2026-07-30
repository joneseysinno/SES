use serde::Serializer;

use super::Rational;

#[derive(serde::Serialize)]
struct RationalRep {
    num: i64,
    den: i64,
}

impl serde::Serialize for Rational {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RationalRep {
            num: self.num(),
            den: self.den(),
        }
        .serialize(serializer)
    }
}
