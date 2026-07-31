use dioxus::prelude::*;
use ses_ui::{page_pods, Label, LabelDef, PageCtx, PodDescriptor, PodKind};

#[component]
pub fn ProposalEditorPage(ctx: PageCtx) -> Element {
    let pods = vec![PodDescriptor::stable(1, PodKind::Section, "Proposal")];

    rsx! {
        div { class: "ses-page ses-page-proposal-editor",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![(
                    1,
                    rsx! {
                        Label {
                            def: LabelDef::new("Author and revise proposals — placeholder.").muted(),
                        }
                    },
                )],
            )}
        }
    }
}
