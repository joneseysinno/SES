use serde::{Deserialize, Serialize};

use crate::codec::SesPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CodeEdition {
    #[default]
    Aci318_19,
    Aci318_25,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UnitSystemPref {
    #[default]
    Imperial,
    Si,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RiskCategory {
    #[default]
    Ii,
    Iii,
    Iv,
}

/// Ordered code-stack entry: address in the `codes` space.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeStackEntry {
    pub code_seq: u64,
}

/// v0.2 design basis: ordered code stack + optional amendment overlay branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DesignBasis {
    pub code_stack: Vec<CodeStackEntry>,
    pub amendment_branch: Option<String>,
    pub display_units: UnitSystemPref,
    pub sds_milli: i64,
    pub sd1_milli: i64,
    pub seismic_design_category: String,
    pub risk_category: RiskCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectStatus {
    #[default]
    Draft,
    Active,
    Issued,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    pub project_number: String,
    pub client: String,
    pub address: String,
    pub design_basis: DesignBasis,
    pub engineer_of_record: String,
    pub status: ProjectStatus,
    pub created_utc: i64,
}

impl SesPayload for Project {
    const SCHEMA_VERSION: u8 = 1;
}
