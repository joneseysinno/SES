use serde::{Deserialize, Serialize};
use ses_engineer::Quantity;

use crate::codec::SesPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ResultKind {
    #[default]
    Check,
    Derivation,
    Routing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CheckStatus {
    #[default]
    Pass,
    Fail,
    NotApplicable,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputPin {
    pub space: u64,
    pub address: Vec<u64>,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CheckResult {
    pub result_kind: ResultKind,
    pub provision_addr: [u64; 6],
    pub title: String,
    pub demand: Quantity,
    pub capacity: Quantity,
    pub ratio_micro: i64,
    pub status: CheckStatus,
    pub governing_combo: Option<String>,
    pub narrative: String,
    pub inputs_used: Vec<InputPin>,
}

impl SesPayload for CheckResult {
    const SCHEMA_VERSION: u8 = 1;
}
