use crate::project::payloads::{ColumnDef, ColumnId, Task};
use crate::shared::{DateRange, MilestoneId, ProjectId, TaskId};

#[derive(Debug, Clone, Default)]
pub struct NewTaskParams {
    pub project_id: ProjectId,
    pub title: String,
}

#[derive(Debug, Clone, Default)]
pub struct TaskPatch {
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewTimeEntryParams {
    pub project_id: ProjectId,
    pub task_id: Option<TaskId>,
    pub minutes: crate::shared::Minutes,
    pub billable: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NewDocRefParams {
    pub project_id: ProjectId,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct NewMilestoneParams {
    pub project_id: ProjectId,
    pub title: String,
    pub target_utc: i64,
}

#[derive(Debug, Clone, Default)]
pub struct TaskFilter;

pub enum ProjectCommand {
    CreateTask(NewTaskParams),
    UpdateTask { id: TaskId, patch: TaskPatch },
    MoveTask {
        id: TaskId,
        column_id: ColumnId,
        order: u32,
    },
    ToggleChecklistItem { task_id: TaskId, index: usize },
    DeleteTask(TaskId),
    LogTime(NewTimeEntryParams),
    AddDocRef(NewDocRefParams),
    CreateMilestone(NewMilestoneParams),
    SetMilestoneActual { id: MilestoneId, actual_utc: i64 },
    AddColumn {
        project_id: ProjectId,
        column: ColumnDef,
    },
    UpdateColumn {
        project_id: ProjectId,
        column: ColumnDef,
    },
    ReorderColumns {
        project_id: ProjectId,
        order: Vec<ColumnId>,
    },
    RemoveColumn {
        project_id: ProjectId,
        id: ColumnId,
        reassign_to: ColumnId,
    },
    ResetBoardToFactory { project_id: ProjectId },
}

pub enum ProjectQuery {
    ListTasks {
        project_id: ProjectId,
        filter: TaskFilter,
    },
    GetTask(TaskId),
    GetBoardConfig(ProjectId),
    ListTimeEntries {
        project_id: ProjectId,
        range: DateRange,
    },
    ListDocs(ProjectId),
    ListMilestones(ProjectId),
    Progress(ProjectId),
}

/// Reassign every task in `from` to `to`, returning updated tasks.
pub fn reassign_column(tasks: &[Task], from: &ColumnId, to: &ColumnId) -> Vec<Task> {
    tasks
        .iter()
        .cloned()
        .map(|mut t| {
            if t.column_id.0 == from.0 {
                t.column_id = to.clone();
            }
            t
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::payloads::board_config::BoardConfig;
    use crate::project::payloads::Priority;
    use crate::shared::Minutes;

    fn sample_task(id: u64, column: &str) -> Task {
        Task {
            id: TaskId::from_raw(id),
            project_id: ProjectId::from_raw(1),
            title: format!("T{id}"),
            description: String::new(),
            column_id: ColumnId::new(column),
            estimate: Minutes(60),
            assignee: None,
            due_utc: None,
            priority: Priority::Normal,
            checklist: vec![],
            order: id as u32,
            created_utc: 0,
            completed_utc: None,
        }
    }

    #[test]
    fn remove_column_reassigns_without_orphans() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let tasks = vec![
            sample_task(1, "blocked"),
            sample_task(2, "backlog"),
        ];
        let updated = reassign_column(&tasks, &ColumnId::new("blocked"), &ColumnId::new("backlog"));
        let recomputed = compute_after(&board, &updated);
        assert_eq!(recomputed.orphan_task_count, 0);
    }

    fn compute_after(board: &BoardConfig, tasks: &[Task]) -> crate::project::progress::ProjectProgress {
        crate::project::progress::compute(board, tasks, &[])
    }
}
