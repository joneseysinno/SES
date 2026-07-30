use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProjectDocsPage(ctx: PageCtx) -> Element {
    let project = ctx.binding_get("project_id").unwrap_or("—");
    rsx! {
        div { class: "ses-page ses-page-project-docs",
            h2 { "Project Documents" }
            p { class: "ses-muted", "Project {project} · document register placeholder." }
            ul {
                li { "Calc Package Rev A" }
                li { "Structural Drawings S1.0" }
                li { "Geotech Report" }
            }
        }
    }
}
