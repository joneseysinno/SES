use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProjectAnalysisPage(ctx: PageCtx) -> Element {
    let project = ctx.binding_get("project_id").unwrap_or("—");
    rsx! {
        div { class: "ses-page ses-page-project-analysis",
            h2 { "Project Analysis" }
            p { class: "ses-muted", "Project {project} · launch point into engineering crates." }
        }
    }
}
