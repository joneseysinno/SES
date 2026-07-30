use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn TaskDetailPage(ctx: PageCtx) -> Element {
    let project = ctx.binding_get("project_id").unwrap_or("—");
    rsx! {
        div { class: "ses-page ses-page-task-detail",
            h2 { "Task Detail" }
            p { class: "ses-muted", "Project {project} · placeholder task detail view." }
        }
    }
}
