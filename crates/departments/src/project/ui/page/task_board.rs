use crate::project::bridge::{
    NewBoardCardParams, NewTaskParams, ProjectCommand, ProjectQuery,
};
use crate::project::payloads::{BoardCard, ColumnDef, ColumnId, Task};
use crate::shared::ui::progress_tone;
use crate::shared::{BoardCardId, ProjectId, TaskId};
use crate::store::{use_dept_store, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, use_shell, Kanban, KanbanCardKind, KanbanColumn, KanbanDef, Label, LabelDef, PageCtx,
    PodDescriptor, PodKind, ProgressBar, ProgressDef, SpecificCardSubtask, SpecificCardTask,
    SpecificKanbanCardDef,
};

#[component]
pub fn TaskBoardPage(ctx: PageCtx) -> Element {
    let mut store = use_dept_store();
    let mut shell = use_shell();
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw);

    let Some(project_id) = project_id else {
        let pods = vec![PodDescriptor::stable(1, PodKind::Anchor, "Board")];
        return rsx! {
            div { class: "ses-page ses-page-task-board",
                {page_pods(
                    pods,
                    ctx.pod_layout.clone(),
                    vec![(
                        1,
                        rsx! {
                            Label {
                                def: LabelDef::new("Bind a project_id to open the task board.")
                                    .muted(),
                            }
                        },
                    )],
                )}
            }
        };
    };

    let s = store.read();
    let board = match s.query_project(ProjectQuery::GetBoardConfig(project_id)) {
        Ok(ProjectQueryResult::Board(b)) => b,
        _ => {
            let pods = vec![PodDescriptor::stable(1, PodKind::Anchor, "Board")];
            return rsx! {
                div { class: "ses-page ses-page-task-board",
                    {page_pods(
                        pods,
                        ctx.pod_layout.clone(),
                        vec![(
                            1,
                            rsx! {
                                Label {
                                    def: LabelDef::new(format!(
                                        "No board for project {project_id}."
                                    ))
                                    .muted(),
                                }
                            },
                        )],
                    )}
                }
            };
        }
    };
    let cards: Vec<BoardCard> = match s.query_project(ProjectQuery::ListBoardCards { project_id }) {
        Ok(ProjectQueryResult::BoardCards(c)) => c,
        _ => vec![],
    };
    let tasks: Vec<Task> = match s.query_project(ProjectQuery::ListTasks {
        project_id,
        filter: Default::default(),
    }) {
        Ok(ProjectQueryResult::Tasks(t)) => t,
        _ => vec![],
    };
    let progress = match s.query_project(ProjectQuery::Progress(project_id)) {
        Ok(ProjectQueryResult::Progress(p)) => p,
        _ => crate::project::progress::ProjectProgress::zero(),
    };
    drop(s);

    let tone = progress_tone(progress.tone());

    let columns: Vec<KanbanColumn> = board
        .columns
        .iter()
        .map(|c| KanbanColumn {
            id: c.id.0.clone(),
            title: c.title.clone(),
            accent: c.accent.clone(),
            limit: c.limit,
        })
        .collect();

    let kanban_cards: Vec<KanbanCardKind> = cards
        .iter()
        .map(|card| {
            let card_tasks: Vec<SpecificCardTask> = tasks
                .iter()
                .filter(|t| t.card_id == card.id)
                .map(|t| SpecificCardTask {
                    id: t.id.0.to_string(),
                    title: t.title.clone(),
                    done: t.completed_utc.is_some(),
                    estimate: Some(t.estimate.as_hours_display()),
                    assignee: t.assignee.clone(),
                    subtasks: t
                        .checklist
                        .iter()
                        .enumerate()
                        .map(|(index, item)| SpecificCardSubtask {
                            index,
                            label: item.label.clone(),
                            done: item.done,
                        })
                        .collect(),
                })
                .collect();
            KanbanCardKind::Specific(SpecificKanbanCardDef {
                id: card.id.0.to_string(),
                column_id: card.column_id.0.clone(),
                title: card.title.clone(),
                order: card.order,
                tasks: card_tasks,
                allow_edit: true,
            })
        })
        .collect();

    let def = KanbanDef {
        columns,
        cards: kanban_cards,
        on_move: Some("move-card".into()),
        on_open: None,
        allow_add: false,
        allow_add_column: true,
    };

    let first_col = board
        .columns
        .first()
        .map(|c| c.id.clone())
        .unwrap_or_else(|| ColumnId::new("proposals"));

    let pods = vec![
        PodDescriptor::stable(1, PodKind::Summary, "Progress"),
        PodDescriptor::stable(2, PodKind::Anchor, "Board"),
    ];

    rsx! {
        div { class: "ses-page ses-page-task-board",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![
                    (
                        1,
                        rsx! {
                            ProgressBar {
                                def: ProgressDef {
                                    fraction: progress.fraction(),
                                    caption: Some(format!(
                                        "{}/{} tasks · {}",
                                        progress.done_task_count,
                                        progress.total_task_count,
                                        progress.completed_estimate.as_hours_display()
                                    )),
                                    secondary_fraction: Some(progress.spent_fraction()),
                                    tone,
                                },
                            }
                        },
                    ),
                    (
                        2,
                        rsx! {
                            button {
                                class: "ses-ghost",
                                r#type: "button",
                                onclick: move |_| {
                                    let n = store
                                        .read()
                                        .board_cards
                                        .values()
                                        .filter(|c| c.project_id == project_id)
                                        .count()
                                        + 1;
                                    if let Err(e) = store.write().execute_project(
                                        ProjectCommand::CreateBoardCard(NewBoardCardParams {
                                            project_id,
                                            title: format!("Card group {n}"),
                                            column_id: first_col.clone(),
                                        }),
                                    ) {
                                        shell.write().status_message =
                                            format!("Add card failed: {e}");
                                    }
                                },
                                "+ New card group"
                            }
                            Kanban {
                                def,
                                on_move: move |(card_id, col_id): (String, String)| {
                                    let Ok(raw) = card_id.parse::<u64>() else { return };
                                    let id = BoardCardId::from_raw(raw);
                                    let order = store
                                        .read()
                                        .board_cards
                                        .get(&id)
                                        .map(|c| c.order)
                                        .unwrap_or(0);
                                    if let Err(e) =
                                        store.write().execute_project(ProjectCommand::MoveBoardCard {
                                            id,
                                            column_id: ColumnId::new(col_id),
                                            order,
                                        })
                                    {
                                        shell.write().status_message = format!("Move failed: {e}");
                                    }
                                },
                                on_add_column: move |col: KanbanColumn| {
                                    let order = store
                                        .read()
                                        .boards
                                        .get(&project_id)
                                        .map(|b| b.columns.len() as u16)
                                        .unwrap_or(0);
                                    if let Err(e) =
                                        store.write().execute_project(ProjectCommand::AddColumn {
                                            project_id,
                                            column: ColumnDef {
                                                id: ColumnId::new(col.id),
                                                title: col.title,
                                                counts_complete: false,
                                                is_exception: false,
                                                accent: col.accent,
                                                limit: col.limit,
                                                order,
                                            },
                                        })
                                    {
                                        shell.write().status_message =
                                            format!("Add column failed: {e}");
                                    } else {
                                        shell.write().status_message = "Column added".into();
                                    }
                                },
                                on_toggle_task: move |(_card_id, task_id): (String, String)| {
                                    let Ok(raw) = task_id.parse::<u64>() else { return };
                                    let _ = store.write().execute_project(
                                        ProjectCommand::ToggleTaskComplete {
                                            task_id: TaskId::from_raw(raw),
                                        },
                                    );
                                },
                                on_toggle_subtask:
                                    move |(_card_id, task_id, index): (String, String, usize)| {
                                        let Ok(raw) = task_id.parse::<u64>() else { return };
                                        let _ = store.write().execute_project(
                                            ProjectCommand::ToggleChecklistItem {
                                                task_id: TaskId::from_raw(raw),
                                                index,
                                            },
                                        );
                                    },
                                on_add_task: move |card_id: String| {
                                    let Ok(raw) = card_id.parse::<u64>() else { return };
                                    let n = store
                                        .read()
                                        .tasks
                                        .values()
                                        .filter(|t| t.card_id.0 == raw)
                                        .count()
                                        + 1;
                                    if let Err(e) = store.write().execute_project(
                                        ProjectCommand::CreateTask(NewTaskParams {
                                            project_id,
                                            card_id: BoardCardId::from_raw(raw),
                                            title: format!("Task {n}"),
                                        }),
                                    ) {
                                        shell.write().status_message =
                                            format!("Add task failed: {e}");
                                    }
                                },
                                on_add_subtask: move |(_card_id, task_id): (String, String)| {
                                    let Ok(raw) = task_id.parse::<u64>() else { return };
                                    let n = store
                                        .read()
                                        .tasks
                                        .get(&TaskId::from_raw(raw))
                                        .map(|t| t.checklist.len() + 1)
                                        .unwrap_or(1);
                                    if let Err(e) = store.write().execute_project(
                                        ProjectCommand::AddChecklistItem {
                                            task_id: TaskId::from_raw(raw),
                                            label: format!("Subtask {n}"),
                                        },
                                    ) {
                                        shell.write().status_message =
                                            format!("Add subtask failed: {e}");
                                    }
                                },
                            }
                        },
                    ),
                ],
            )}
        }
    }
}
