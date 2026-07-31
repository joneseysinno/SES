use dioxus::prelude::*;
use ses_ui::{page_pods, Label, LabelDef, PageCtx, PodDescriptor, PodKind};

#[component]
pub fn TaskDetailPage(ctx: PageCtx) -> Element {
    let pods = vec![PodDescriptor::stable(1, PodKind::Section, "Task")];
    let msg = match ctx.binding_get("project_id") {
        Some(id) => format!("Project {id} · placeholder task detail view."),
        None => "Bind a project_id to open task detail.".into(),
    };

    rsx! {
        div { class: "ses-page ses-page-task-detail",
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
