use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            class: "ses-home",
            h1 { "Structural Engineering Solutions" }
            p { "SES — scaffolding" }
        }
    }
}
