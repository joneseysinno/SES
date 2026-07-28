//! Global top bar — SES brand toggles app ribbon; workspace tabs.

use crate::pod::app_ribbon::AppRibbon;
use crate::workspace::WorkspaceBar;
use dioxus::prelude::*;

#[component]
pub fn TopBarPod() -> Element {
    let mut ribbon_open = use_signal(|| false);

    rsx! {
        div { class: "ses-chrome",
            header { class: "ses-top-bar",
                button {
                    class: if ribbon_open() { "ses-brand ses-active" } else { "ses-brand" },
                    title: "App menu",
                    onclick: move |_| {
                        ribbon_open.toggle();
                    },
                    "SES"
                }
                WorkspaceBar {}
            }
            if ribbon_open() {
                AppRibbon {}
            }
        }
    }
}
