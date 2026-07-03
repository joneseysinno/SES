use serde::{Deserialize, Serialize};

use crate::codec::SesPayload;
use crate::payload::project::CodeEdition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnalysisStatus {
    #[default]
    Complete,
    Failed,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AnalysisSummary {
    pub n_checks: u32,
    pub n_pass: u32,
    pub n_fail: u32,
    pub governing_check_seq: u64,
    pub governing_ratio_micro: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AnalysisRun {
    pub started_utc: i64,
    pub engine_version: String,
    pub code_edition: CodeEdition,
    pub status: AnalysisStatus,
    pub summary: AnalysisSummary,
    /// Blake3 hash of the assembled provision graph (pipeline §7 stage 4).
    pub graph_hash: String,
}

impl SesPayload for AnalysisRun {
    const SCHEMA_VERSION: u8 = 1;
}
