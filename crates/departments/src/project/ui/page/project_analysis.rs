use dioxus::prelude::*;
use ses_ui::{page_pods, Label, LabelDef, PageCtx, PodDescriptor, PodKind};

#[component]
pub fn ProjectAnalysisPage(ctx: PageCtx) -> Element {
    let pods = vec![PodDescriptor::stable(1, PodKind::Section, "Analysis")];
    let msg = match ctx.binding_get("project_id") {
        Some(id) => format!("Project {id} · launch point into engineering crates."),
        None => "Bind a project_id to open analysis.".into(),
    };

    rsx! {
        div { class: "ses-page ses-page-project-analysis",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![(
                    1,
                    rsx! {
                        Label {
                            def: LabelDef::new(msg).muted(),
                        }
                    },
                )],
            )}
        }
    }
}
