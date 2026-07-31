use crate::project_management::bridge::{
    NewProjectParams, NewProposalParams, ProjectMgmtCommand, ProjectMgmtQuery,
};
use crate::project_management::payloads::ProjectPhase;
use crate::project::progress::ProgressTone as DeptProgressTone;
use crate::store::{use_dept_store, MgmtQueryResult, StoreEffect};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, use_shell, BadgeDef, BadgeTone, Kanban, KanbanCardKind, KanbanColumn, KanbanDef,
    PageCtx, PodDescriptor, PodKind, ProgressDef, ProgressTone, SummaryKanbanCardDef,
};

#[component]
pub fn ProjectBoardPage(ctx: PageCtx) -> Element {
    let mut store = use_dept_store();
    let mut shell = use_shell();

    // Drain workspace top-bar actions into store commands.
    {
        let actions = shell.write().take_top_bar_actions();
        for action in actions {
            match action.as_str() {
                "new-project" => {
                    let n = store.read().projects.len() + 1;
                    match store.write().execute_mgmt(ProjectMgmtCommand::CreateProject(
                        NewProjectParams {
                            name: format!("New Project {n}"),
                            number: format!("2026-{n:03}"),
                        },
                    )) {
                        Ok(_) => shell.write().status_message = "Created project".into(),
                        Err(e) => {
                            shell.write().status_message = format!("Create project failed: {e}")
                        }
                    }
                }
                "new-proposal" => {
                    let n = store.read().projects.len() + 1;
                    let create = store.write().execute_mgmt(ProjectMgmtCommand::CreateProject(
                        NewProjectParams {
                            name: format!("Proposal {n}"),
                            number: format!("P-2026-{n:03}"),
                        },
                    ));
                    match create {
                        Ok(_) => {
                            let pid = store
                                .read()
                                .projects
                                .keys()
                                .copied()
                                .max_by_key(|id| id.0)
                                .expect("just created");
                            let _ = store.write().execute_mgmt(ProjectMgmtCommand::SetPhase {
                                id: pid,
                                phase: ProjectPhase::Proposal,
                            });
                            match store.write().execute_mgmt(ProjectMgmtCommand::CreateProposal(
                                NewProposalParams {
                                    project_id: pid,
                                    scope: String::new(),
                                },
                            )) {
                                Ok(_) => {
                                    shell.write().status_message =
                                        "Created proposal project + draft".into()
                                }
                                Err(e) => {
                                    shell.write().status_message =
                                        format!("Create proposal failed: {e}")
                                }
                            }
                        }
                        Err(e) => {
                            shell.write().status_message = format!("Create proposal failed: {e}")
                        }
                    }
                }
                other => {
                    shell.write().status_message = format!("Unknown top-bar action: {other}");
                }
            }
        }
    }

    let (projects, progress_map) = {
        let s = store.read();
        let projects = match s.query_mgmt(ProjectMgmtQuery::ListAll {
            filter: Default::default(),
        }) {
            Ok(MgmtQueryResult::Projects(p)) => p,
            _ => vec![],
        };
        let progress_map = match s.query_mgmt(ProjectMgmtQuery::PortfolioProgress) {
            Ok(MgmtQueryResult::PortfolioProgress(m)) => m,
            _ => Default::default(),
        };
        (projects, progress_map)
    };

    let columns: Vec<KanbanColumn> = ProjectPhase::all()
        .iter()
        .map(|phase| KanbanColumn {
            id: phase.column_id().into(),
            title: phase.title().into(),
            accent: None,
            limit: None,
        })
        .collect();

    let cards: Vec<KanbanCardKind> = projects
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let progress = progress_map.get(&p.id);
            let fraction = progress.map(|pr| pr.fraction()).unwrap_or(0.0);
            let spent = progress.map(|pr| pr.spent_fraction());
            let tone = progress
                .map(|pr| map_tone(pr.tone()))
                .unwrap_or(ProgressTone::Neutral);
            let caption = progress.map(|pr| {
                format!(
                    "{} / {} est · {} tasks left",
                    pr.completed_estimate.as_hours_display(),
                    pr.total_estimate.as_hours_display(),
                    pr.open_task_count
                )
            });
            KanbanCardKind::Summary(SummaryKanbanCardDef {
                id: p.id.0.to_string(),
                column_id: p.phase.column_id().into(),
                title: p.name.clone(),
                subtitle: Some(format!("{} · {}", p.number, p.client.name)),
                badges: vec![BadgeDef {
                    label: format!("{:?}", p.status),
                    tone: BadgeTone::Neutral,
                }],
                progress: Some(ProgressDef {
                    fraction,
                    caption,
                    secondary_fraction: spent,
                    tone,
                }),
                metrics: vec![],
                order: i as u32,
            })
        })
        .collect();

    let def = KanbanDef {
        columns,
        cards,
        on_move: Some("set-phase".into()),
        on_open: Some("open-project".into()),
        allow_add: false,
        allow_add_column: false,
    };

    let pods = vec![PodDescriptor::stable(1, PodKind::Anchor, "Board")];

    rsx! {
        div { class: "ses-page ses-page-project-board",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![(
                    1,
                    rsx! {
                        Kanban {
                            def,
                            on_move: move |(card_id, col_id): (String, String)| {
                                let Ok(raw) = card_id.parse::<u64>() else { return };
                                let Some(phase) = ProjectPhase::all()
                                    .iter()
                                    .copied()
                                    .find(|p| p.column_id() == col_id)
                                else {
                                    return;
                                };
                                let id = crate::shared::ProjectId::from_raw(raw);
                                if let Err(e) = store.write().execute_mgmt(ProjectMgmtCommand::SetPhase { id, phase }) {
                                    shell.write().status_message = format!("Move failed: {e}");
                                }
                            },
                            on_open: move |card_id: String| {
                                let Ok(raw) = card_id.parse::<u64>() else { return };
                                let project_id = crate::shared::ProjectId::from_raw(raw);
                                let effect = store.write().execute_mgmt(
                                    ProjectMgmtCommand::OpenProjectWorkspace { project_id },
                                );
                                match effect {
                                    Ok(StoreEffect::OpenWorkspace(ws)) => {
                                        shell.write().add_workspace(ws);
                                        shell.write().status_message = "Opened project workspace".into();
                                    }
                                    Ok(_) => {}
                                    Err(e) => {
                                        shell.write().status_message = format!("Open failed: {e}");
                                    }
                                }
                            },
                        }
                    },
                )],
            )}
        }
    }
}

fn map_tone(t: DeptProgressTone) -> ProgressTone {
    match t {
        DeptProgressTone::Neutral => ProgressTone::Neutral,
        DeptProgressTone::Good => ProgressTone::Good,
        DeptProgressTone::Warn => ProgressTone::Warn,
        DeptProgressTone::Over => ProgressTone::Over,
    }
}
