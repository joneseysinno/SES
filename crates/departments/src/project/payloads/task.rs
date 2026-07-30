use crate::project::payloads::board_config::ColumnId;
use crate::shared::{Minutes, ProjectId, TaskId};
use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

/// A checkable unit of work — the source of truth for progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub title: String,
    pub description: String,
    pub column_id: ColumnId,
    pub estimate: Minutes,
    pub assignee: Option<String>,
    pub due_utc: Option<i64>,
    pub priority: Priority,
    pub checklist: Vec<ChecklistItem>,
    pub order: u32,
    pub created_utc: i64,
    pub completed_utc: Option<i64>,
}

impl Versioned for Task {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Task {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["assignee"];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChecklistItem {
    pub label: String,
    pub done: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Priority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}
