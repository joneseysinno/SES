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

use crate::project::pages::task_board::mock_tasks;
use crate::project::payloads::BoardConfig;
use crate::project::progress::compute;
use crate::shared::ProjectId;
use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProjectOverviewPage(ctx: PageCtx) -> Element {
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw)
        .unwrap_or_else(ProjectId::new);

    let board = BoardConfig::factory(project_id);
    let tasks = mock_tasks(project_id, &board);
    let progress = compute(&board, &tasks, &[]);
    let pct = (progress.fraction() * 100.0).round() as i32;

    rsx! {
        div { class: "ses-page ses-page-project-overview",
            h2 { "Project Overview" }
            p { class: "ses-muted",
                "Progress {pct}% · {progress.open_task_count} tasks open"
            }
        }
    }
}
