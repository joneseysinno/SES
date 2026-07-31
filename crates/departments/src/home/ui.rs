pub mod page;
pub mod pod;
pub mod workspace;

use dioxus::prelude::*;
use ses_shell::ModuleId;
use ses_ui::{PageCtx, SesModuleUi};

use crate::home::MODULE_ID_STR;
use page::{Page, ProjectsOverviewPage, TimecardPage};

pub struct HomeUi;

impl SesModuleUi for HomeUi {
    fn module_id(&self) -> ModuleId {
        ModuleId::new(MODULE_ID_STR)
    }

    fn render_page(&self, page_id: &ses_shell::PageId, ctx: &PageCtx) -> Element {
        match Page::from_id(page_id.as_str()) {
            Some(Page::ProjectsOverview) => rsx! { ProjectsOverviewPage { ctx: ctx.clone() } },
            Some(Page::Timecard) => rsx! { TimecardPage { ctx: ctx.clone() } },
            None => {
                let other = page_id.as_str();
                rsx! {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "Unknown home page: {other}" }
                    }
                }
            }
        }
    }
}
