use crate::project::payloads::board_config::ColumnId;
use crate::shared::{BoardCardId, ProjectId};
use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

/// A named group of tasks on a project Kanban board (e.g. "Design Criteria").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardCard {
    pub id: BoardCardId,
    pub project_id: ProjectId,
    pub column_id: ColumnId,
    pub title: String,
    pub order: u32,
}

impl Versioned for BoardCard {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for BoardCard {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["project manager"];
}
