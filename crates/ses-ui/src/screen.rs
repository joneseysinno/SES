//! Screen — full-app viewport shell.

use crate::context::use_shell;
use crate::page::PageArea;
use crate::pod::{StatusBarPod, TopBarPod};
use dioxus::prelude::*;
use ses_shell::ops::effective_layout;

#[component]
pub fn Screen() -> Element {
    let shell = use_shell();

    let (layout, top_bar, landmarks) = {
        let s = shell.read();
        match s.active() {
            Some(ws) => (
                Some(effective_layout(ws)),
                ws.top_bar.clone(),
                ws.landmarks.clone(),
            ),
            None => (None, None, Vec::new()),
        }
    };

    rsx! {
        div { class: "ses-screen",
            TopBarPod {}
            div { class: "ses-workspace-area",
                if let Some(node) = layout {
                    PageArea { node, top_bar, landmarks }
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
