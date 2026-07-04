//! Serde helpers for `ses-core` types (orphan-rule safe).

use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use ses_core::{Dim, Rational, UnitId};

#[derive(Serialize, Deserialize)]
struct RationalRep {
    num: i64,
    den: i64,
}

pub mod rational {
    use super::*;

    pub fn serialize<S: Serializer>(value: &Rational, serializer: S) -> Result<S::Ok, S::Error> {
        RationalRep {
            num: value.num(),
            den: value.den(),
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Rational, D::Error> {
        let rep = RationalRep::deserialize(deserializer)?;
        Rational::new(rep.num, rep.den).map_err(serde::de::Error::custom)
    }
}

pub mod unit_id {
    use super::*;

    pub fn serialize<S: Serializer>(value: &UnitId, serializer: S) -> Result<S::Ok, S::Error> {
        value.0.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<UnitId, D::Error> {
        Ok(UnitId(u32::deserialize(deserializer)?))
    }
}

#[derive(Serialize, Deserialize)]
struct DimRep {
    force: i8,
    length: i8,
    time: i8,
    temperature: i8,
}

pub mod dim {
    use super::*;

    pub fn serialize<S: Serializer>(value: &Dim, serializer: S) -> Result<S::Ok, S::Error> {
        DimRep {
            force: value.force,
            length: value.length,
            time: value.time,
            temperature: value.temp,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Dim, D::Error> {
        let rep = DimRep::deserialize(deserializer)?;
        Ok(Dim::new(rep.force, rep.length, rep.time, rep.temperature))
    }
}

/// Local newtype so `Expr` can derive serde over rationals.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WireRational(pub Rational);

impl fmt::Debug for WireRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for WireRational {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.num().hash(state);
        self.0.den().hash(state);
    }
}

impl Deref for WireRational {
    type Target = Rational;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rational> for WireRational {
    fn from(value: Rational) -> Self {
        Self(value)
    }
}

impl Serialize for WireRational {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        rational::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for WireRational {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        rational::deserialize(deserializer).map(Self)
    }
}
