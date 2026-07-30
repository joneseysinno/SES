use serde::{Deserialize, Serialize};

/// Default ACI edition for new projects when none is specified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CodeEdition {
    /// ACI 318-19.
    #[default]
    Aci318_19,
    /// ACI 318-25.
    Aci318_25,
}

/// Default display unit preference for new design bases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UnitSystemPref {
    /// US customary units.
    #[default]
    Imperial,
    /// SI units.
    Si,
}

/// Default risk category for new design bases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RiskCategory {
    /// Risk Category II.
    #[default]
    Ii,
    /// Risk Category III.
    Iii,
    /// Risk Category IV.
    Iv,
}

/// Ordered code-stack entry: address in the `codes` space.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeStackEntry {
    /// Sequence in the codes space.
    pub code_seq: u64,
}

/// v0.2 design basis: ordered code stack + optional amendment overlay branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignBasis {
    /// Ordered adopting codes.
    pub code_stack: Vec<CodeStackEntry>,
    /// Optional amendment overlay branch label.
    pub amendment_branch: Option<String>,
    /// Display unit preference.
    pub display_units: UnitSystemPref,
    /// SDS in milli-units (exact integer storage).
    pub sds_milli: i64,
    /// SD1 in milli-units (exact integer storage).
    pub sd1_milli: i64,
    /// Seismic design category label.
    pub seismic_design_category: String,
    /// Risk category.
    pub risk_category: RiskCategory,
}

/// Default lifecycle status for newly created projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectStatus {
    /// Draft, not yet issued.
    #[default]
    Draft,
    /// Active design.
    Active,
    /// Issued for construction.
    Issued,
    /// Archived.
    Archived,
}
