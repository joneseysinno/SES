use crate::project::payloads::{BoardConfig, Task, TimeEntry};
use crate::shared::Minutes;
use ses_core::Ephemeral;

/// Visual tone for progress bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressTone {
    #[default]
    Neutral,
    Good,
    Warn,
    Over,
}

/// Derived progress for one project — never persisted.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectProgress {
    pub total_estimate: Minutes,
    pub completed_estimate: Minutes,
    pub spent_billable: Minutes,
    pub spent_nonbillable: Minutes,
    pub total_task_count: u32,
    pub done_task_count: u32,
    pub open_task_count: u32,
    pub blocked_task_count: u32,
    pub orphan_task_count: u32,
}

impl Ephemeral for ProjectProgress {
    const DERIVED_FROM: &'static [&'static str] = &["task", "time-entry", "board-config"];
}

impl ProjectProgress {
    pub fn zero() -> Self {
        Self {
            total_estimate: Minutes(0),
            completed_estimate: Minutes(0),
            spent_billable: Minutes(0),
            spent_nonbillable: Minutes(0),
            total_task_count: 0,
            done_task_count: 0,
            open_task_count: 0,
            blocked_task_count: 0,
            orphan_task_count: 0,
        }
    }

    pub fn spent_total(&self) -> Minutes {
        Minutes(
            self.spent_billable
                .0
                .saturating_add(self.spent_nonbillable.0),
        )
    }

    pub fn fraction(&self) -> f32 {
        if self.total_estimate.0 == 0 {
            if self.total_task_count == 0 {
                return 0.0;
            }
            return self.done_task_count as f32 / self.total_task_count as f32;
        }
        (self.completed_estimate.0 as f32 / self.total_estimate.0 as f32).clamp(0.0, 1.0)
    }

    pub fn burn_fraction(&self) -> f32 {
        if self.total_estimate.0 == 0 {
            return 0.0;
        }
        (self.spent_billable.0 as f32 / self.total_estimate.0 as f32).clamp(0.0, 1.5)
    }

    /// Alias for UI layers that name the secondary bar "spent".
    pub fn spent_fraction(&self) -> f32 {
        self.burn_fraction()
    }

    pub fn nonbillable_share(&self) -> f32 {
        let total = self.spent_total().0;
        if total == 0 {
            return 0.0;
        }
        self.spent_nonbillable.0 as f32 / total as f32
    }

    pub fn tone(&self) -> ProgressTone {
        if self.blocked_task_count > 0 || self.orphan_task_count > 0 {
            return ProgressTone::Warn;
        }
        let done = self.fraction();
        let burn = self.burn_fraction();
        if burn > done + 0.15 {
            ProgressTone::Over
        } else if done >= 1.0 {
            ProgressTone::Good
        } else {
            ProgressTone::Neutral
        }
    }

    pub fn is_unestimated(&self) -> bool {
        self.total_estimate.0 == 0 && self.total_task_count > 0
    }
}

/// Compute the rollup for one project.
pub fn compute(board: &BoardConfig, tasks: &[Task], entries: &[TimeEntry]) -> ProjectProgress {
    let mut p = ProjectProgress::zero();

    for t in tasks {
        p.total_estimate.0 = p.total_estimate.0.saturating_add(t.estimate.0);
        p.total_task_count = p.total_task_count.saturating_add(1);

        match board.column(&t.column_id) {
            Some(col) if col.counts_complete && !col.is_exception => {
                p.completed_estimate.0 = p.completed_estimate.0.saturating_add(t.estimate.0);
                p.done_task_count = p.done_task_count.saturating_add(1);
            }
            Some(col) => {
                p.open_task_count = p.open_task_count.saturating_add(1);
                if col.is_exception {
                    p.blocked_task_count = p.blocked_task_count.saturating_add(1);
                }
            }
            None => {
                p.open_task_count = p.open_task_count.saturating_add(1);
                p.orphan_task_count = p.orphan_task_count.saturating_add(1);
            }
        }
    }

    for e in entries {
        if e.billable {
            p.spent_billable.0 = p.spent_billable.0.saturating_add(e.minutes.0);
        } else {
            p.spent_nonbillable.0 = p.spent_nonbillable.0.saturating_add(e.minutes.0);
        }
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::payloads::board_config::{ColumnDef, ColumnId};
    use crate::project::payloads::Priority;
    use crate::shared::{ProjectId, TaskId, TimeEntryId};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    fn task(
        id: u64,
        column: &str,
        estimate_hours: u32,
        project_id: ProjectId,
    ) -> Task {
        Task {
            id: TaskId::from_raw(id),
            project_id,
            title: format!("Task {id}"),
            description: String::new(),
            column_id: ColumnId::new(column),
            estimate: Minutes::from_hours(estimate_hours),
            assignee: None,
            due_utc: None,
            priority: Priority::Normal,
            checklist: vec![],
            order: id as u32,
            created_utc: now(),
            completed_utc: None,
        }
    }

    fn board_with_extra_complete(project_id: ProjectId) -> BoardConfig {
        let mut board = BoardConfig::factory(project_id);
        board.columns.push(ColumnDef {
            id: ColumnId::new("shipped"),
            title: "Shipped".into(),
            counts_complete: true,
            is_exception: false,
            accent: None,
            limit: None,
            order: 6,
        });
        board
    }

    #[test]
    fn empty_project_is_zero() {
        let board = BoardConfig::factory(ProjectId::from_raw(1));
        let p = compute(&board, &[], &[]);
        assert_eq!(p.fraction(), 0.0);
        assert_eq!(p.burn_fraction(), 0.0);
        assert!(!p.is_unestimated());
    }

    #[test]
    fn all_done_is_one() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let tasks = vec![
            task(1, "done", 4, pid),
            task(2, "done", 8, pid),
        ];
        let p = compute(&board, &tasks, &[]);
        assert!((p.fraction() - 1.0).abs() < f32::EPSILON);
        assert_eq!(p.tone(), ProgressTone::Good);
    }

    #[test]
    fn weighted_not_count_weighted() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let mut open = Vec::new();
        for i in 0..10 {
            open.push(task(i + 2, "backlog", 1, pid));
        }
        let mut tasks = vec![task(1, "done", 80, pid)];
        tasks.extend(open);
        let p = compute(&board, &tasks, &[]);
        assert!((p.fraction() - 0.888).abs() < 0.01);
        assert!((p.fraction() - 0.09).abs() > 0.01);
    }

    #[test]
    fn zero_estimate_fallback_uses_task_count() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let mut t1 = task(1, "done", 0, pid);
        t1.estimate = Minutes(0);
        let mut t2 = task(2, "backlog", 0, pid);
        t2.estimate = Minutes(0);
        let p = compute(&board, &[t1, t2], &[]);
        assert!((p.fraction() - 0.5).abs() < f32::EPSILON);
        assert!(p.is_unestimated());
    }

    #[test]
    fn blocked_task_forces_warn() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let tasks = vec![task(1, "blocked", 4, pid)];
        let p = compute(&board, &tasks, &[]);
        assert_eq!(p.blocked_task_count, 1);
        assert_eq!(p.tone(), ProgressTone::Warn);
    }

    #[test]
    fn burn_over_completion_forces_over() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let tasks = vec![task(1, "in-progress", 100, pid)];
        let entries = vec![TimeEntry {
            id: TimeEntryId::from_raw(1),
            project_id: pid,
            task_id: Some(TaskId::from_raw(1)),
            who: "eng".into(),
            minutes: Minutes::from_hours(50),
            date_utc: now(),
            note: String::new(),
            billable: true,
        }];
        let p = compute(&board, &tasks, &entries);
        assert_eq!(p.tone(), ProgressTone::Over);
    }

    #[test]
    fn saturating_add_at_max_does_not_panic() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let mut t = task(1, "done", 0, pid);
        t.estimate = Minutes(u32::MAX);
        let entries = vec![TimeEntry {
            id: TimeEntryId::from_raw(1),
            project_id: pid,
            task_id: None,
            who: "eng".into(),
            minutes: Minutes(u32::MAX),
            date_utc: now(),
            note: String::new(),
            billable: true,
        }];
        let p = compute(&board, &[t], &entries);
        assert_eq!(p.total_estimate.0, u32::MAX);
        assert_eq!(p.spent_billable.0, u32::MAX);
    }

    #[test]
    fn two_complete_columns_both_count() {
        let pid = ProjectId::from_raw(1);
        let board = board_with_extra_complete(pid);
        let tasks = vec![
            task(1, "done", 10, pid),
            task(2, "shipped", 5, pid),
            task(3, "backlog", 5, pid),
        ];
        let p = compute(&board, &tasks, &[]);
        assert_eq!(p.completed_estimate, Minutes::from_hours(15));
        assert!((p.fraction() - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn complete_and_exception_counts_as_neither() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig {
            project_id: pid,
            columns: vec![ColumnDef {
                id: ColumnId::new("weird"),
                title: "Weird".into(),
                counts_complete: true,
                is_exception: true,
                accent: None,
                limit: None,
                order: 0,
            }],
        };
        let tasks = vec![task(1, "weird", 10, pid)];
        let p = compute(&board, &tasks, &[]);
        assert_eq!(p.done_task_count, 0);
        assert_eq!(p.open_task_count, 1);
        assert_eq!(p.fraction(), 0.0);
    }

    #[test]
    fn orphan_counts_open_and_warns() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let tasks = vec![task(1, "removed-column", 4, pid)];
        let p = compute(&board, &tasks, &[]);
        assert_eq!(p.orphan_task_count, 1);
        assert_eq!(p.open_task_count, 1);
        assert_eq!(p.done_task_count, 0);
        assert_eq!(p.tone(), ProgressTone::Warn);
    }

    #[test]
    fn renaming_column_title_does_not_change_numbers() {
        let pid = ProjectId::from_raw(1);
        let mut board = BoardConfig::factory(pid);
        let tasks = vec![task(1, "done", 10, pid), task(2, "backlog", 10, pid)];
        let before = compute(&board, &tasks, &[]);

        if let Some(col) = board.columns.iter_mut().find(|c| c.id.0 == "done") {
            col.title = "Finished".into();
        }
        let after = compute(&board, &tasks, &[]);
        assert_eq!(before, after);
    }

    #[test]
    fn reordering_columns_does_not_change_numbers() {
        let pid = ProjectId::from_raw(1);
        let mut board = BoardConfig::factory(pid);
        let tasks = vec![task(1, "done", 10, pid), task(2, "backlog", 10, pid)];
        let before = compute(&board, &tasks, &[]);

        board.columns.reverse();
        let after = compute(&board, &tasks, &[]);
        assert_eq!(before, after);
    }

    #[test]
    fn nonbillable_never_affects_burn() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let tasks = vec![task(1, "in-progress", 10, pid)];
        let entries = vec![TimeEntry {
            id: TimeEntryId::from_raw(1),
            project_id: pid,
            task_id: Some(TaskId::from_raw(1)),
            who: "eng".into(),
            minutes: Minutes::from_hours(8),
            date_utc: now(),
            note: String::new(),
            billable: false,
        }];
        let p = compute(&board, &tasks, &entries);
        assert_eq!(p.burn_fraction(), 0.0);
        assert_eq!(p.spent_nonbillable, Minutes::from_hours(8));
        assert!((p.nonbillable_share() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn spent_total_sums_both_kinds() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let entries = vec![
            TimeEntry {
                id: TimeEntryId::from_raw(1),
                project_id: pid,
                task_id: None,
                who: "a".into(),
                minutes: Minutes::from_hours(2),
                date_utc: now(),
                note: String::new(),
                billable: true,
            },
            TimeEntry {
                id: TimeEntryId::from_raw(2),
                project_id: pid,
                task_id: None,
                who: "b".into(),
                minutes: Minutes::from_hours(3),
                date_utc: now(),
                note: String::new(),
                billable: false,
            },
        ];
        let p = compute(&board, &[], &entries);
        assert_eq!(p.spent_total(), Minutes::from_hours(5));
    }

    #[test]
    fn zero_entries_no_divide_by_zero() {
        let pid = ProjectId::from_raw(1);
        let board = BoardConfig::factory(pid);
        let p = compute(&board, &[], &[]);
        assert_eq!(p.burn_fraction(), 0.0);
        assert_eq!(p.nonbillable_share(), 0.0);
    }
}
