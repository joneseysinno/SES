use crate::shared::{DocRefId, ProjectId};
use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

/// Reference to a document — bytes live elsewhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocRef {
    pub id: DocRefId,
    pub project_id: ProjectId,
    pub title: String,
    pub category: DocCategory,
    pub uri: String,
    pub revision: String,
    pub added_utc: i64,
    pub added_by: String,
}

impl Versioned for DocRef {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for DocRef {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["project manager"];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DocCategory {
    Drawing,
    Calc,
    Report,
    Correspondence,
    Submittal,
    #[default]
    Other,
}
