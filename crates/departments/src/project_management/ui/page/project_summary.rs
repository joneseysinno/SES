use crate::project::bridge::ProjectQuery;
use crate::project::progress::ProgressTone as DeptProgressTone;
use crate::project_management::bridge::ProjectMgmtQuery;
use crate::shared::ProjectId;
use crate::store::{use_dept_store, MgmtQueryResult, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, Badge, BadgeDef, BadgeTone, Label, LabelDef, PageCtx, PodDescriptor, PodKind,
    ProgressBar, ProgressDef, ProgressTone,
};

#[component]
pub fn ProjectSummaryPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let bound_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw);

    let s = store.read();
    let project = if let Some(id) = bound_id {
        match s.query_mgmt(ProjectMgmtQuery::GetProject(id)) {
            Ok(MgmtQueryResult::Project(p)) => Some(p),
            _ => None,
        }
    } else {
        None
    };

    let pods = vec![PodDescriptor::stable(1, PodKind::Summary, "Summary")];

    let body = match project {
        Some(p) => {
            let progress = match s.query_project(ProjectQuery::Progress(p.id)) {
                Ok(ProjectQueryResult::Progress(pr)) => pr,
                _ => crate::project::progress::ProjectProgress::zero(),
            };
            let tone = map_tone(progress.tone());
            rsx! {
                Label {
                    def: LabelDef::new(format!("{} · {}", p.name, p.number)),
                }
                Label {
                    def: LabelDef::new(format!(
                        "Client: {} · Manager: {}",
                        p.client.name, p.manager
                    ))
                    .muted(),
                }
                Badge {
                    def: BadgeDef {
                        label: format!("{:?}", p.status),
                        tone: BadgeTone::Neutral,
                    },
                }
                ProgressBar {
                    def: ProgressDef {
                        fraction: progress.fraction(),
                        caption: Some(format!(
                            "{} / {} est · {} open",
                            progress.completed_estimate.as_hours_display(),
                            progress.total_estimate.as_hours_display(),
                            progress.open_task_count
                        )),
                        secondary_fraction: Some(progress.spent_fraction()),
                        tone,
                    },
                }
            }
        }
        None => rsx! {
            Label {
                def: LabelDef::new("Bind a project_id to open the summary.").muted(),
            }
        },
    };

    rsx! {
        div { class: "ses-page ses-page-project-summary",
            {page_pods(pods, ctx.pod_layout.clone(), vec![(1, body)])}
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
