use crate::project::bridge::ProjectQuery;
use crate::shared::{DateRange, Minutes, ProjectId};
use crate::store::{use_dept_store, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, DataTable, Label, LabelDef, Metric, MetricDef, PageCtx, PodDescriptor, PodKind,
    TableColumn, TableDef, TableRow,
};

#[component]
pub fn TimeTrackingPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw);

    let pods = vec![
        PodDescriptor::stable(1, PodKind::Summary, "Week total"),
        PodDescriptor::stable(2, PodKind::Scroller, "Entries"),
    ];

    let (summary_body, entries_body) = match project_id {
        Some(project_id) => {
            let entries = match store.read().query_project(ProjectQuery::ListTimeEntries {
                project_id,
                range: DateRange {
                    start_utc: i64::MIN / 4,
                    end_utc: i64::MAX / 4,
                },
            }) {
                Ok(ProjectQueryResult::TimeEntries(e)) => e,
                _ => vec![],
            };
            let total_minutes: u32 = entries.iter().map(|e| e.minutes.0).sum();
            let week_total = Minutes(total_minutes).as_hours_display();
            (
                rsx! {
                    Metric {
                        def: MetricDef {
                            label: "Logged".into(),
                            value: week_total,
                            delta: None,
                            delta_up: None,
                        },
                    }
                },
                rsx! {
                    DataTable {
                        def: TableDef {
                            columns: vec![
                                TableColumn {
                                    id: "who".into(),
                                    title: "Who".into(),
                                    sortable: true,
                                },
                                TableColumn {
                                    id: "hours".into(),
                                    title: "Hours".into(),
                                    sortable: true,
                                },
                                TableColumn {
                                    id: "billable".into(),
                                    title: "Billable".into(),
                                    sortable: true,
                                },
                                TableColumn {
                                    id: "note".into(),
                                    title: "Note".into(),
                                    sortable: false,
                                },
                            ],
                            rows: entries
                                .into_iter()
                                .map(|e| TableRow {
                                    id: e.id.0.to_string(),
                                    cells: vec![
                                        e.who,
                                        e.minutes.as_hours_display(),
                                        if e.billable { "Yes".into() } else { "No".into() },
                                        e.note,
                                    ],
                                })
                                .collect(),
                        },
                    }
                },
            )
        }
        None => (
            rsx! {
                Label {
                    def: LabelDef::new("Bind a project_id to open time tracking.").muted(),
                }
            },
            rsx! {
                Label {
                    def: LabelDef::new("No entries.").muted(),
                }
            },
        ),
    };

    rsx! {
        div { class: "ses-page ses-page-time-tracking",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![(1, summary_body), (2, entries_body)],
            )}
        }
    }
}
