use crate::shared::{DocRefId, ProjectId};
use serde::{Deserialize, Serialize};

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
