use crate::project::bridge::ProjectQuery;
use crate::project_management::bridge::{ProjectFilter, ProjectMgmtCommand, ProjectMgmtQuery};
use crate::project_management::portfolio;
use crate::shared::ui::apply_open_workspace;
use crate::shared::{now_utc, week_range_utc, Minutes, ProjectId};
use crate::store::{use_dept_store, MgmtQueryResult, ProjectQueryResult, StoreEffect};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, use_shell, DataTable, Label, LabelDef, Metric, MetricDef, PageCtx, PodDescriptor,
    PodKind, TableColumn, TableDef, TableRow,
};

/// Home's projects summary — read-only rollup of every active project.
/// Every number here comes from `project_management::portfolio::aggregate`;
/// this page computes nothing of its own.
#[component]
pub fn ProjectsOverviewPage(ctx: PageCtx) -> Element {
    let mut store = use_dept_store();
    let mut shell = use_shell();

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

    let week = week_range_utc(now_utc());
    let hours_this_week: u32 = {
        let s = store.read();
        projects
            .iter()
            .map(|p| {
                match s.query_project(ProjectQuery::ListTimeEntries {
                    project_id: p.id,
                    range: week,
                }) {
                    Ok(ProjectQueryResult::TimeEntries(entries)) => {
                        entries.iter().map(|e| e.minutes.0).sum::<u32>()
                    }
                    _ => 0,
                }
            })
            .sum()
    };

    let mut open_project = move |raw_id: String| {
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
    };

    let body = if projects.is_empty() {
        rsx! {
            Label {
                def: LabelDef::new("No projects yet. Create one from Project Management.").muted(),
            }
        }
    } else {
        let def = TableDef {
            columns: vec![
                TableColumn { id: "number".into(), title: "Number".into(), sortable: true },
                TableColumn { id: "name".into(), title: "Name".into(), sortable: true },
                TableColumn { id: "client".into(), title: "Client".into(), sortable: true },
                TableColumn { id: "phase".into(), title: "Phase".into(), sortable: true },
                TableColumn { id: "percent".into(), title: "% Complete".into(), sortable: true },
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
                            pct,
                            target,
                        ],
                    }
                })
                .collect(),
        };
        rsx! {
            DataTable {
                def,
                on_row_activate: move |id: String| open_project(id),
            }
        }
    };

    let pods = vec![
        PodDescriptor::stable(1, PodKind::Summary, "Portfolio"),
        PodDescriptor::stable(2, PodKind::Scroller, "Active Projects"),
    ];

    rsx! {
        div { class: "ses-page ses-page-home-projects-overview",
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
                                    label: "Hours this week".into(),
                                    value: Minutes(hours_this_week).as_hours_display(),
                                    delta: None,
                                    delta_up: None,
                                },
                            }
                        },
                    ),
                    (2, body),
                ],
            )}
        }
    }
}
