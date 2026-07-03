use serde::{Deserialize, Serialize};
use ses_engineer::{DimensionSignature, Rational, UnitSystem};

use crate::codec::SesPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Unit {
    pub symbol: String,
    pub name: String,
    pub dim: DimensionSignature,
    pub ratio_to_pivot: Rational,
    pub system: UnitSystem,
    pub display_precision: u8,
}

impl SesPayload for Unit {
    const SCHEMA_VERSION: u8 = 1;
}
