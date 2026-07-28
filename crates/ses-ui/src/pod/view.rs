//! 3D viewport placeholder.

use dioxus::prelude::*;

#[component]
pub fn ViewPod() -> Element {
    rsx! {
        div { class: "ses-pod ses-view-pod",
            div { class: "ses-view-label", "3D Viewport" }
        }
    }
}
