use crate::shared::{Minutes, ProjectId, TaskId, TimeEntryId};
use serde::{Deserialize, Serialize};

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
