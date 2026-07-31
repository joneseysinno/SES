use crate::shared::{Minutes, ProjectId, TaskId, TimeEntryId};
use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

/// Time actually logged against a project or task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub project_id: ProjectId,
    pub task_id: Option<TaskId>,
    pub who: String,
    pub minutes: Minutes,
    pub date_utc: i64,
    pub note: String,
    pub billable: bool,
}

impl Versioned for TimeEntry {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for TimeEntry {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["assignee"];
}
