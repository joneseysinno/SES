use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

use crate::payload::project::CodeEdition;

/// Default status for a completed analysis run record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnalysisStatus {
    /// Run completed successfully.
    #[default]
    Complete,
    /// Run failed.
    Failed,
    /// Superseded by a later run.
    Superseded,
}

/// Aggregate check statistics for an analysis run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisSummary {
    /// Total checks evaluated.
    pub n_checks: u32,
    /// Checks that passed.
    pub n_pass: u32,
    /// Checks that failed.
    pub n_fail: u32,
    /// Governing check sequence number.
    pub governing_check_seq: u64,
    /// Governing demand/capacity ratio in micro-units.
    pub governing_ratio_micro: i64,
}

/// Analysis run header in the `analyses` space (ses-code-pipeline §7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisRun {
    /// Start timestamp (UTC seconds).
    pub started_utc: i64,
    /// Engine semver string.
    pub engine_version: String,
    /// Code edition used.
    pub code_edition: CodeEdition,
    /// Run status.
    pub status: AnalysisStatus,
    /// Aggregate statistics.
    pub summary: AnalysisSummary,
    /// Blake3 hash of the assembled provision graph (pipeline §7 stage 4).
    pub graph_hash: String,
}

impl Versioned for AnalysisRun {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for AnalysisRun {
    const KIND: TestimonyKind = TestimonyKind::Emitted;
    const WITNESSES: &'static [&'static str] = &["analysis-run"];
}
