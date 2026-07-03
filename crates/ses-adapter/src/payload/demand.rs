use serde::{Deserialize, Serialize};
use ses_engineer::Quantity;

use crate::codec::SesPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandSource {
    Manual,
    Imported { origin: String },
}

impl Default for DemandSource {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Demand {
    pub combo_label: String,
    pub pu: Quantity,
    pub vu: Quantity,
    pub mu: Quantity,
    pub delta_u: Quantity,
    pub hsx: Quantity,
    pub source: DemandSource,
}

impl SesPayload for Demand {
    const SCHEMA_VERSION: u8 = 1;
}
