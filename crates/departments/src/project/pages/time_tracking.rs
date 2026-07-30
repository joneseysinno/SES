use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn TimeTrackingPage(ctx: PageCtx) -> Element {
    let project = ctx.binding_get("project_id").unwrap_or("—");
    rsx! {
        div { class: "ses-page ses-page-time-tracking",
            h2 { "Time Tracking" }
            p { class: "ses-muted", "Project {project} · week total and entry table placeholder." }
        }
    }
}
