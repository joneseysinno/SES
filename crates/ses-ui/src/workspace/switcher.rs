//! Add / reset workspace actions (dup/remove live on tab context menu).

use crate::context::use_shell;
use dioxus::prelude::*;
use ses_shell::{PageNode, PodDescriptor, PodKind, WorkspaceDef, default_shell};

#[component]
pub fn WorkspaceSwitcher() -> Element {
    let mut shell = use_shell();

    rsx! {
        button {
            class: "ses-workspace-add ses-ghost",
            title: "Add workspace",
            onclick: move |_| {
                let blank = WorkspaceDef::new(
                    "Workspace",
                    PageNode::leaf(PodDescriptor::new(PodKind::View, "core-ui")),
                );
                shell.write().add_workspace(blank);
                shell.write().status_message = "Workspace added".into();
            },
            "+"
        }
        button {
            class: "ses-ghost",
            title: "Reset to defaults",
            onclick: move |_| {
                let active_name = shell
                    .read()
                    .active()
                    .map(|w| w.name.clone())
                    .unwrap_or_default();
                *shell.write() = default_shell();
                shell.write().status_message = format!("Reset defaults (was {active_name})");
            },
            "↺"
        }
    }
}
