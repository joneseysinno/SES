use crate::project::payloads::BoardConfig;
use crate::project::progress::compute;
use crate::project_management::bridge::mock_projects;
use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProjectSummaryPage(ctx: PageCtx) -> Element {
    let project = mock_projects().into_iter().next();
    rsx! {
        div { class: "ses-page ses-page-project-summary",
            h2 { "Project Summary" }
            if let Some(p) = project {
                {
                    let board = BoardConfig::factory(p.id);
                    let progress = compute(&board, &[], &[]);
                    rsx! {
                        p { "{p.name} · {p.number}" }
                        p { class: "ses-muted",
                            "Client: {p.client.name} · Manager: {p.manager}"
                        }
                        p { "Progress: {(progress.fraction() * 100.0).round() as i32}%" }
                    }
                }
            } else {
                p { class: "ses-muted", "No project selected." }
            }
        }
    }
}
