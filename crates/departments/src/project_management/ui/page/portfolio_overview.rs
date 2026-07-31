use crate::project_management::bridge::{ProjectFilter, ProjectMgmtCommand, ProjectMgmtQuery};
use crate::project_management::payloads::ProjectRecord;
use crate::project_management::portfolio;
use crate::project::progress::ProjectProgress;
use crate::shared::ui::{apply_open_workspace, progress_tone};
use crate::shared::ProjectId;
use crate::store::{use_dept_store, DeptStoreCtx, MgmtQueryResult, StoreEffect};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, use_shell, BarChart, BarChartDef, ChartRow, DataTable, Metric, MetricDef, PageCtx,
    PodDescriptor, PodKind, ShellCtx, TableColumn, TableDef, TableRow,
};

use super::{NewProjectDialog, NewProposalDialog};

/// Shared by the chart's `on_select` and the table's `on_row_activate` — a
/// plain fn, not a closure, so both event handlers can `move` their own
/// `Copy` handles into it without fighting over unique ownership.
fn open_project(mut store: DeptStoreCtx, mut shell: ShellCtx, raw_id: String) {
    let Ok(raw) = raw_id.parse::<u64>() else { return };
    let project_id = ProjectId::from_raw(raw);
    let effect = store
        .write()
        .execute_mgmt(ProjectMgmtCommand::OpenProjectWorkspace { project_id });
    match effect {
        Ok(StoreEffect::OpenWorkspace(ws)) => {
            let mut s = shell.write();
            apply_open_workspace(&mut s, ws);
        }
        Ok(_) => {}
        Err(e) => shell.write().status_message = format!("Open failed: {e}"),
    }
}

/// The Project Management portfolio page — progress summary + active list.
/// Owns the top-bar action drain for `new-project` / `new-proposal`.
#[component]
pub fn PortfolioOverviewPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let mut shell = use_shell();
    let mut show_new_project = use_signal(|| false);
    let mut show_new_proposal = use_signal(|| false);

    {
        let actions = shell.write().take_top_bar_actions_for(ctx.workspace_id);
        for action in actions {
            match action.as_str() {
                "new-project" => show_new_project.set(true),
                "new-proposal" => show_new_proposal.set(true),
                other => {
                    shell.write().status_message = format!("Unknown top-bar action: {other}");
                }
            }
        }
    }

    let (projects, progress_map) = {
        let s = store.read();
        let projects = match s.query_mgmt(ProjectMgmtQuery::ListAll {
            filter: ProjectFilter::active(),
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

    let metrics = portfolio::aggregate(projects.iter().filter_map(|p| progress_map.get(&p.id)));

    let mut risk_rows: Vec<(ProjectRecord, ProjectProgress)> = projects
        .iter()
        .filter_map(|p| progress_map.get(&p.id).map(|pr| (p.clone(), pr.clone())))
        .collect();
    portfolio::risk_sort(&mut risk_rows);

    let chart_rows: Vec<ChartRow> = risk_rows
        .iter()
        .map(|(p, pr)| ChartRow {
            id: p.id.0.to_string(),
            label: p.number.clone(),
            sublabel: Some(p.name.clone()),
            value: pr.fraction(),
            secondary: Some(pr.spent_fraction()),
            tone: progress_tone(pr.tone()),
        })
        .collect();

    let table_def = TableDef {
        columns: vec![
            TableColumn { id: "number".into(), title: "Number".into(), sortable: true },
            TableColumn { id: "name".into(), title: "Name".into(), sortable: true },
            TableColumn { id: "client".into(), title: "Client".into(), sortable: true },
            TableColumn { id: "phase".into(), title: "Phase".into(), sortable: true },
            TableColumn { id: "manager".into(), title: "Manager".into(), sortable: true },
            TableColumn { id: "percent".into(), title: "%".into(), sortable: true },
            TableColumn { id: "target".into(), title: "Target".into(), sortable: false },
        ],
        rows: projects
            .iter()
            .map(|p| {
                let pct = progress_map
                    .get(&p.id)
                    .map(|pr| format!("{:.0}%", pr.fraction() * 100.0))
                    .unwrap_or_else(|| "—".into());
                let target = if p.target_finish_utc.is_some() {
                    "Set".to_string()
                } else {
                    "—".to_string()
                };
                TableRow {
                    id: p.id.0.to_string(),
                    cells: vec![
                        p.number.clone(),
                        p.name.clone(),
                        p.client.name.clone(),
                        p.phase.title().into(),
                        p.manager.clone(),
                        pct,
                        target,
                    ],
                }
            })
            .collect(),
    };

    let pods = vec![
        PodDescriptor::stable(1, PodKind::Summary, "Portfolio Progress"),
        PodDescriptor::stable(2, PodKind::Scroller, "Active Projects"),
    ];

    rsx! {
        div { class: "ses-page ses-page-portfolio-overview",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![
                    (
                        1,
                        rsx! {
                            Metric {
                                def: MetricDef {
                                    label: "Active projects".into(),
                                    value: metrics.active_count.to_string(),
                                    delta: None,
                                    delta_up: None,
                                },
                            }
                            Metric {
                                def: MetricDef {
                                    label: "Weighted completion".into(),
                                    value: format!("{:.0}%", metrics.weighted_fraction * 100.0),
                                    delta: None,
                                    delta_up: None,
                                },
                            }
                            Metric {
                                def: MetricDef {
                                    label: "At risk".into(),
                                    value: metrics.at_risk_count.to_string(),
                                    delta: None,
                                    delta_up: None,
                                },
                            }
                            BarChart {
                                def: BarChartDef {
                                    rows: chart_rows,
                                    gridlines: vec![0.25, 0.5, 0.75],
                                    caption: None,
                                    max_rows: Some(12),
                                },
                                on_select: move |id: String| open_project(store, shell, id),
                            }
                        },
                    ),
                    (
                        2,
                        rsx! {
                            DataTable {
                                def: table_def,
                                on_row_activate: move |id: String| open_project(store, shell, id),
                            }
                        },
                    ),
                ],
            )}
            if show_new_project() {
                NewProjectDialog { on_close: move |_| show_new_project.set(false) }
            }
            if show_new_proposal() {
                NewProposalDialog { on_close: move |_| show_new_proposal.set(false) }
            }
        }
    }
}
