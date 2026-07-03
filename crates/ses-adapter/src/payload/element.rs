use serde::{Deserialize, Serialize};
use ses_engineer::Quantity;

use crate::codec::SesPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SpecialWallGeometry {
    pub lw: Quantity,
    pub hw: Quantity,
    pub tw: Quantity,
    pub story_heights: Vec<Quantity>,
    pub n_curtains: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementKind {
    SpecialWall(SpecialWallGeometry),
}

impl Default for ElementKind {
    fn default() -> Self {
        Self::SpecialWall(SpecialWallGeometry {
            lw: Quantity::new(
                ses_engineer::Rational::one(),
                ses_engineer::UnitId(0),
                "",
            ),
            hw: Quantity::new(
                ses_engineer::Rational::one(),
                ses_engineer::UnitId(0),
                "",
            ),
            tw: Quantity::new(
                ses_engineer::Rational::one(),
                ses_engineer::UnitId(0),
                "",
            ),
            story_heights: Vec::new(),
            n_curtains: 2,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Element {
    pub kind: ElementKind,
    pub label: String,
    pub grid_location: String,
    pub notes: String,
}

impl SesPayload for Element {
    const SCHEMA_VERSION: u8 = 1;
}
