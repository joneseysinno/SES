pub mod page;
pub mod pod;
pub mod workspace;

use dioxus::prelude::*;
use ses_shell::ModuleId;
use ses_ui::{PageCtx, SesModuleUi};

use crate::project::MODULE_ID_STR;
use page::{
    Page, ProjectAnalysisPage, ProjectDocsPage, ProjectOverviewPage, ProjectTimelinePage,
    TaskBoardPage, TaskDetailPage, TimeTrackingPage,
};

pub struct ProjectUi;

impl SesModuleUi for ProjectUi {
    fn module_id(&self) -> ModuleId {
        ModuleId::new(MODULE_ID_STR)
    }

    fn render_page(&self, page_id: &ses_shell::PageId, ctx: &PageCtx) -> Element {
        match Page::from_id(page_id.as_str()) {
            Some(Page::TaskBoard) => rsx! { TaskBoardPage { ctx: ctx.clone() } },
            Some(Page::TaskDetail) => rsx! { TaskDetailPage { ctx: ctx.clone() } },
            Some(Page::ProjectDocs) => rsx! { ProjectDocsPage { ctx: ctx.clone() } },
            Some(Page::ProjectAnalysis) => rsx! { ProjectAnalysisPage { ctx: ctx.clone() } },
            Some(Page::TimeTracking) => rsx! { TimeTrackingPage { ctx: ctx.clone() } },
            Some(Page::ProjectTimeline) => rsx! { ProjectTimelinePage { ctx: ctx.clone() } },
            Some(Page::ProjectOverview) => rsx! { ProjectOverviewPage { ctx: ctx.clone() } },
            None => {
                let other = page_id.as_str();
                rsx! {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "Unknown project page: {other}" }
                    }
                }
            }
        }
    }
}
