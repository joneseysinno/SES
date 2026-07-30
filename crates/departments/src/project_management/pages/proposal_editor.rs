use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProposalEditorPage(ctx: PageCtx) -> Element {
    rsx! {
        div { class: "ses-page ses-page-proposal-editor",
            h2 { "Proposal Editor" }
            p { class: "ses-muted", "Author and revise proposals — placeholder." }
        }
    }
}
