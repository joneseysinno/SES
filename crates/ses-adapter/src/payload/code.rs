use serde::{Deserialize, Serialize};

use crate::codec::SesPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CodeRole {
    #[default]
    Adopting,
    Loads,
    MaterialDesign,
    Prescriptive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Code {
    pub family: String,
    pub edition_label: String,
    pub role: CodeRole,
    pub issuing_body: String,
    pub effective_from_utc: Option<i64>,
    pub effective_to_utc: Option<i64>,
}

impl SesPayload for Code {
    const SCHEMA_VERSION: u8 = 1;
}
