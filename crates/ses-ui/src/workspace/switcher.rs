use crate::context::{use_modules, use_shell};
use crate::workspace::reset_to_factory;
use dioxus::prelude::*;
use ses_shell::{PageDescriptor, PageNode, WorkspaceDef};

#[component]
pub fn WorkspaceSwitcher() -> Element {
    let mut shell = use_shell();
    let modules = use_modules();

    rsx! {
        button {
            class: "ses-workspace-add ses-ghost",
            title: "Add workspace",
            onclick: move |_| {
                let blank = WorkspaceDef::new(
                    "Workspace",
                    PageNode::leaf(PageDescriptor::new("core-ui", "view")),
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
                let mods = modules.read();
                *shell.write() = reset_to_factory(&mods);
                shell.write().status_message = format!("Reset defaults (was {active_name})");
            },
            "↺"
        }
    }
}
