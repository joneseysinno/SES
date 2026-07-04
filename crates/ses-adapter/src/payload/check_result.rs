use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};
use ses_engineer::Quantity;

/// Default result kind for provision checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ResultKind {
    /// Strength or serviceability check.
    #[default]
    Check,
    /// Intermediate derivation step.
    Derivation,
    /// Routing decision.
    Routing,
}

/// Default status for a passing check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CheckStatus {
    /// Demand ≤ capacity.
    #[default]
    Pass,
    /// Demand > capacity.
    Fail,
    /// Not applicable to this element.
    NotApplicable,
    /// Passed with warning.
    Warning,
}

/// Provenance pin for an input value used in a check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputPin {
    /// Source space id.
    pub space: u64,
    /// Address coordinates.
    pub address: Vec<u64>,
    /// Observed revision.
    pub revision: u64,
}

/// Individual check result in the `check_results` space (ses-code-pipeline §7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    /// Result classification.
    pub result_kind: ResultKind,
    /// Governing provision address.
    pub provision_addr: [u64; 6],
    /// Check title.
    pub title: String,
    /// Demand quantity.
    pub demand: Quantity,
    /// Capacity quantity.
    pub capacity: Quantity,
    /// Demand/capacity ratio in micro-units.
    pub ratio_micro: i64,
    /// Verdict status.
    pub status: CheckStatus,
    /// Governing load combination label.
    pub governing_combo: Option<String>,
    /// Rendered narrative.
    pub narrative: String,
    /// Input provenance pins.
    pub inputs_used: Vec<InputPin>,
}

impl Versioned for CheckResult {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for CheckResult {
    const KIND: TestimonyKind = TestimonyKind::Emitted;
    const WITNESSES: &'static [&'static str] = &["analysis-run"];
}
