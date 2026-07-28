//! Global status bar.

use crate::context::{use_shell, use_user};
use dioxus::prelude::*;

#[component]
pub fn StatusBarPod() -> Element {
    let shell = use_shell();
    let user = use_user();

    let message = shell.read().status_message.clone();
    let user_name = user.read().display_name.clone();
    let ws_name = shell
        .read()
        .active()
        .map(|w| w.name.clone())
        .unwrap_or_else(|| "—".into());

    rsx! {
        footer { class: "ses-status-bar",
            span { "{message}" }
            span {
                "{user_name} · {ws_name}"
            }
        }
    }
}
