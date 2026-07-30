use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProjectTimelinePage(ctx: PageCtx) -> Element {
    let project = ctx.binding_get("project_id").unwrap_or("—");
    rsx! {
        div { class: "ses-page ses-page-project-timeline",
            h2 { "Project Timeline" }
            p { class: "ses-muted", "Project {project} · milestone timeline placeholder." }
        }
    }
}
