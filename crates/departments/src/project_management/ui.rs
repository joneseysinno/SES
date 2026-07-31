pub mod page;
pub mod pod;
pub mod workspace;

use dioxus::prelude::*;
use ses_shell::ModuleId;
use ses_ui::{PageCtx, SesModuleUi};

use crate::project_management::MODULE_ID_STR;
use page::{
    Page, ProjectBoardPage, ProjectListPage, ProjectMetricsPage, ProjectSummaryPage,
    ProposalEditorPage,
};

pub struct ProjectManagementUi;

impl SesModuleUi for ProjectManagementUi {
    fn module_id(&self) -> ModuleId {
        ModuleId::new(MODULE_ID_STR)
    }

    fn render_page(&self, page_id: &ses_shell::PageId, ctx: &PageCtx) -> Element {
        match Page::from_id(page_id.as_str()) {
            Some(Page::ProjectBoard) => rsx! { ProjectBoardPage { ctx: ctx.clone() } },
            Some(Page::ProjectList) => rsx! { ProjectListPage { ctx: ctx.clone() } },
            Some(Page::ProjectSummary) => rsx! { ProjectSummaryPage { ctx: ctx.clone() } },
            Some(Page::ProposalEditor) => rsx! { ProposalEditorPage { ctx: ctx.clone() } },
            Some(Page::ProjectMetrics) => rsx! { ProjectMetricsPage { ctx: ctx.clone() } },
            None => {
                let other = page_id.as_str();
                rsx! {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "Unknown project-mgmt page: {other}" }
                    }
                }
            }
        }
    }
}
