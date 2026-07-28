//! Screen — full-app viewport shell.

use crate::context::use_shell;
use crate::page::PageNodeView;
use crate::pod::{StatusBarPod, TopBarPod};
use dioxus::prelude::*;
use ses_shell::ops::effective_layout;

#[component]
pub fn Screen() -> Element {
    let shell = use_shell();

    let layout = {
        let s = shell.read();
        s.active().map(effective_layout)
    };

    rsx! {
        div { class: "ses-screen",
            TopBarPod {}
            div { class: "ses-workspace-area",
                if let Some(node) = layout {
                    PageNodeView { node, path: vec![] }
                } else {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "No active workspace." }
                    }
                }
            }
            StatusBarPod {}
        }
    }
}
