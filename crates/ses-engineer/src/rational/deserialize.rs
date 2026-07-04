use serde::Deserializer;

use super::Rational;

#[derive(serde::Deserialize)]
struct RationalRep {
    num: i64,
    den: i64,
}

impl<'de> serde::Deserialize<'de> for Rational {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let rep = RationalRep::deserialize(deserializer)?;
        Rational::new(rep.num, rep.den).map_err(serde::de::Error::custom)
    }
}
