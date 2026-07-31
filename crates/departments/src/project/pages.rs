mod project_analysis;
mod project_docs;
mod project_timeline;
mod task_board;
mod task_detail;
mod time_tracking;

pub use project_analysis::ProjectAnalysisPage;
pub use project_docs::ProjectDocsPage;
pub use project_timeline::ProjectTimelinePage;
pub use task_board::TaskBoardPage;
pub use task_detail::TaskDetailPage;
pub use time_tracking::TimeTrackingPage;

use crate::project::bridge::ProjectQuery;
use crate::project::progress::ProgressTone as DeptProgressTone;
use crate::shared::ProjectId;
use crate::store::{use_dept_store, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, Label, LabelDef, PageCtx, PodDescriptor, PodKind, ProgressBar, ProgressDef,
    ProgressTone,
};

#[component]
pub fn ProjectOverviewPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw);

    let pods = vec![PodDescriptor::stable(1, PodKind::Summary, "Overview")];

    let body = match project_id {
        Some(project_id) => {
            let progress = match store.read().query_project(ProjectQuery::Progress(project_id)) {
                Ok(ProjectQueryResult::Progress(p)) => p,
                _ => crate::project::progress::ProjectProgress::zero(),
            };
            let tone = map_tone(progress.tone());
            rsx! {
                ProgressBar {
                    def: ProgressDef {
                        fraction: progress.fraction(),
                        caption: Some(format!(
                            "{} / {} est · {} tasks open",
                            progress.completed_estimate.as_hours_display(),
                            progress.total_estimate.as_hours_display(),
                            progress.open_task_count
                        )),
                        secondary_fraction: Some(progress.spent_fraction()),
                        tone,
                    },
                }
                Label {
                    def: LabelDef::new(format!(
                        "{}/{} tasks done",
                        progress.done_task_count, progress.total_task_count
                    ))
                    .muted(),
                }
            }
        }
        None => rsx! {
            Label {
                def: LabelDef::new("Bind a project_id to open the overview.").muted(),
            }
        },
    };

    rsx! {
        div { class: "ses-page ses-page-project-overview",
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
