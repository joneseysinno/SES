use crate::project::payloads::{BoardConfig, ColumnId, Priority, Task};
use crate::project::progress::{compute, ProgressTone};
use crate::shared::{Minutes, ProjectId, TaskId};
use dioxus::prelude::*;
use ses_ui::PageCtx;

pub fn mock_tasks(project_id: ProjectId, board: &BoardConfig) -> Vec<Task> {
    let cols = ["backlog", "ready", "in-progress", "blocked", "review", "done"];
    cols.iter()
        .enumerate()
        .map(|(i, col)| Task {
            id: TaskId::from_raw(i as u64 + 1),
            project_id,
            title: format!("Sample task — {}", board.column(&ColumnId::new(*col)).map(|c| c.title.as_str()).unwrap_or(col)),
            description: "Placeholder task for scaffold UI.".into(),
            column_id: ColumnId::new(*col),
            estimate: Minutes::from_hours((i as u32 + 1) * 2),
            assignee: Some("Demo User".into()),
            due_utc: None,
            priority: Priority::Normal,
            checklist: vec![],
            order: i as u32,
            created_utc: 1_700_000_000,
            completed_utc: None,
        })
        .collect()
}

#[component]
pub fn TaskBoardPage(ctx: PageCtx) -> Element {
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw)
        .unwrap_or_else(ProjectId::new);

    let board = BoardConfig::factory(project_id);
    let tasks = mock_tasks(project_id, &board);
    let progress = compute(&board, &tasks, &[]);
    let tone = match progress.tone() {
        ProgressTone::Neutral => "neutral",
        ProgressTone::Good => "good",
        ProgressTone::Warn => "warn",
        ProgressTone::Over => "over",
    };

    rsx! {
        div { class: "ses-page ses-page-task-board",
            h2 { "Task Board" }
            p { class: "ses-muted",
                "Project {project_id} · {progress.done_task_count}/{progress.total_task_count} tasks · tone: {tone}"
            }
            div { class: "ses-kanban-preview",
                for col in board.columns.iter() {
                    div { class: "ses-kanban-col-preview", key: "{col.id.0}",
                        h3 { "{col.title}" }
                        ul {
                            for task in tasks.iter().filter(|t| t.column_id.0 == col.id.0) {
                                li { key: "{task.id.0}",
                                    "{task.title} ({task.estimate.as_hours_display()})"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
