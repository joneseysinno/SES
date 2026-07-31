use crate::project::bridge::{NewTimeEntryParams, ProjectCommand, ProjectQuery};
use crate::project_management::bridge::{ProjectFilter, ProjectMgmtQuery};
use crate::shared::{now_utc, week_range_utc, Minutes, ProjectId};
use crate::store::{use_dept_store, MgmtQueryResult, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, use_shell, DataTable, FieldMeta, Metric, MetricDef, NumericalInput, PageCtx,
    PodDescriptor, PodKind, SelectInput, SelectOption, TableColumn, TableDef, TableRow, TextInput,
};

/// This week's hours: a quick-entry form plus a rollup of what the signed-in
/// user has already logged. Every write goes through `project::bridge` — this
/// page owns none of the numbers it shows.
#[component]
pub fn TimecardPage(ctx: PageCtx) -> Element {
    let mut store = use_dept_store();
    let mut shell = use_shell();
    let who = ctx.user.display_name.clone();

    let projects = {
        let s = store.read();
        match s.query_mgmt(ProjectMgmtQuery::ListAll {
            filter: ProjectFilter::active(),
        }) {
            Ok(MgmtQueryResult::Projects(p)) => p,
            _ => vec![],
        }
    };

    let mut selected_project = use_signal(|| {
        projects
            .first()
            .map(|p| p.id.0.to_string())
            .unwrap_or_default()
    });
    let mut hours = use_signal(String::new);
    let mut billable = use_signal(|| true);
    let mut note = use_signal(String::new);
    let mut hours_error = use_signal(|| None::<String>);

    let week = week_range_utc(now_utc());
    let my_entries: Vec<(String, crate::project::payloads::TimeEntry)> = {
        let s = store.read();
        projects
            .iter()
            .flat_map(|p| {
                let entries = match s.query_project(ProjectQuery::ListTimeEntries {
                    project_id: p.id,
                    range: week,
                }) {
                    Ok(ProjectQueryResult::TimeEntries(e)) => e,
                    _ => vec![],
                };
                entries
                    .into_iter()
                    .filter(|e| e.who == who)
                    .map(|e| (p.number.clone(), e))
                    .collect::<Vec<_>>()
            })
            .collect()
    };
    let total_minutes: u32 = my_entries.iter().map(|(_, e)| e.minutes.0).sum();

    let log_time = move |_| {
        let Ok(project_raw) = selected_project.read().parse::<u64>() else {
            hours_error.set(Some("Pick a project".into()));
            return;
        };
        let parsed_hours = hours.read().trim().parse::<f64>().ok();
        let Some(h) = parsed_hours.filter(|h| *h > 0.0) else {
            hours_error.set(Some("Enter hours worked".into()));
            return;
        };

        let result = store.write().execute_project(ProjectCommand::LogTime(NewTimeEntryParams {
            project_id: ProjectId::from_raw(project_raw),
            task_id: None,
            who: who.clone(),
            minutes: Minutes((h * 60.0).round() as u32),
            note: note.read().clone(),
            billable: billable(),
        }));
        match result {
            Ok(_) => {
                shell.write().status_message = "Logged time".into();
                hours.set(String::new());
                note.set(String::new());
                hours_error.set(None);
            }
            Err(e) => shell.write().status_message = format!("Log time failed: {e}"),
        }
    };

    let pods = vec![
        PodDescriptor::stable(1, PodKind::Summary, "This week"),
        PodDescriptor::stable(2, PodKind::Scroller, "My entries"),
    ];

    let entries_table = if my_entries.is_empty() {
        rsx! {
            p { class: "ses-muted", "No entries logged this week." }
        }
    } else {
        rsx! {
            DataTable {
                def: TableDef {
                    columns: vec![
                        TableColumn { id: "project".into(), title: "Project".into(), sortable: true },
                        TableColumn { id: "hours".into(), title: "Hours".into(), sortable: true },
                        TableColumn { id: "billable".into(), title: "Billable".into(), sortable: true },
                        TableColumn { id: "note".into(), title: "Note".into(), sortable: false },
                    ],
                    rows: my_entries
                        .iter()
                        .map(|(number, e)| TableRow {
                            id: e.id.0.to_string(),
                            cells: vec![
                                number.clone(),
                                e.minutes.as_hours_display(),
                                if e.billable { "Yes".into() } else { "No".into() },
                                e.note.clone(),
                            ],
                        })
                        .collect(),
                },
            }
        }
    };

    rsx! {
        div { class: "ses-page ses-page-home-timecard",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![
                    (
                        1,
                        rsx! {
                            Metric {
                                def: MetricDef {
                                    label: "Logged this week".into(),
                                    value: Minutes(total_minutes).as_hours_display(),
                                    delta: None,
                                    delta_up: None,
                                },
                            }
                            if projects.is_empty() {
                                p { class: "ses-muted", "No active projects to log time against." }
                            } else {
                                SelectInput {
                                    meta: FieldMeta::new("timecard-project", "Project"),
                                    value: selected_project(),
                                    options: projects
                                        .iter()
                                        .map(|p| SelectOption {
                                            value: p.id.0.to_string(),
                                            label: format!("{} · {}", p.number, p.name),
                                        })
                                        .collect(),
                                    onchange: move |v: String| selected_project.set(v),
                                }
                                NumericalInput {
                                    id: "timecard-hours".to_string(),
                                    label: "Hours".to_string(),
                                    value: hours(),
                                    error: hours_error(),
                                    oninput: move |v: String| {
                                        hours.set(v);
                                        hours_error.set(None);
                                    },
                                }
                                label { class: "ses-io-field-row",
                                    input {
                                        r#type: "checkbox",
                                        checked: billable(),
                                        onchange: move |e| billable.set(e.checked()),
                                    }
                                    "Billable"
                                }
                                TextInput {
                                    id: "timecard-note".to_string(),
                                    label: "Note".to_string(),
                                    value: note(),
                                    oninput: move |v: String| note.set(v),
                                }
                                button {
                                    r#type: "button",
                                    onclick: log_time,
                                    "Log"
                                }
                            }
                        },
                    ),
                    (2, entries_table),
                ],
            )}
        }
    }
}
