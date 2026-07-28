//! Menu bar placeholder pod — app menus live in the SES logo ribbon.

use dioxus::prelude::*;

#[component]
pub fn MenuBarPod() -> Element {
    rsx! {
        div { class: "ses-pod",
            div { class: "ses-pod-title", "Menu Bar" }
            p { class: "ses-muted",
                "App menus are in the SES logo ribbon (click SES in the top bar)."
            }
        }
    }
}
