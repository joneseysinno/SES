use crate::shared::{MilestoneId, ProjectId, TaskId};
use serde::{Deserialize, Serialize};

/// A dated project milestone with optional gating tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone {
    pub id: MilestoneId,
    pub project_id: ProjectId,
    pub title: String,
    pub target_utc: i64,
    pub actual_utc: Option<i64>,
    pub gating_tasks: Vec<TaskId>,
}
