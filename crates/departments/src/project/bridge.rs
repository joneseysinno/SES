use crate::project::payloads::{BoardCard, ColumnDef, ColumnId};
use crate::shared::{BoardCardId, DateRange, MilestoneId, ProjectId, TaskId};

#[derive(Debug, Clone)]
pub struct NewBoardCardParams {
    pub project_id: ProjectId,
    pub title: String,
    pub column_id: ColumnId,
}

#[derive(Debug, Clone)]
pub struct NewTaskParams {
    pub project_id: ProjectId,
    pub card_id: BoardCardId,
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
    pub who: String,
    pub minutes: crate::shared::Minutes,
    pub note: String,
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
    CreateBoardCard(NewBoardCardParams),
    MoveBoardCard {
        id: BoardCardId,
        column_id: ColumnId,
        order: u32,
    },
    CreateTask(NewTaskParams),
    UpdateTask { id: TaskId, patch: TaskPatch },
    ToggleTaskComplete { task_id: TaskId },
    ToggleChecklistItem { task_id: TaskId, index: usize },
    AddChecklistItem { task_id: TaskId, label: String },
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
    ListBoardCards { project_id: ProjectId },
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

/// Reassign every board card in `from` to `to`.
pub fn reassign_cards(cards: &[BoardCard], from: &ColumnId, to: &ColumnId) -> Vec<BoardCard> {
    cards
        .iter()
        .cloned()
        .map(|mut c| {
            if c.column_id.0 == from.0 {
                c.column_id = to.clone();
            }
            c
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::payloads::board_config::BoardConfig;
    use crate::project::payloads::{Priority, Task};
    use crate::shared::Minutes;

    fn sample_card(id: u64, column: &str) -> BoardCard {
        BoardCard {
            id: BoardCardId::from_raw(id),
            project_id: ProjectId::from_raw(1),
            column_id: ColumnId::new(column),
            title: format!("C{id}"),
            order: id as u32,
        }
    }

    fn sample_task(id: u64, card_id: u64) -> Task {
        Task {
            id: TaskId::from_raw(id),
            project_id: ProjectId::from_raw(1),
            card_id: BoardCardId::from_raw(card_id),
            title: format!("T{id}"),
            description: String::new(),
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
    fn remove_column_reassigns_cards_without_orphans() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let cards = vec![
            sample_card(1, "stamp-review"),
            sample_card(2, "proposals"),
        ];
        let updated = reassign_cards(
            &cards,
            &ColumnId::new("stamp-review"),
            &ColumnId::new("proposals"),
        );
        let tasks = vec![sample_task(1, 1), sample_task(2, 2)];
        let recomputed = crate::project::progress::compute(&board, &updated, &tasks, &[]);
        assert_eq!(recomputed.orphan_task_count, 0);
    }
}
