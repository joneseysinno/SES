//! Properties inspector — key/value rows.

use dioxus::prelude::*;

#[component]
pub fn PropertiesPod() -> Element {
    rsx! {
        div { class: "ses-pod",
            div { class: "ses-pod-title", "Properties" }
            table { class: "ses-props-table",
                tbody {
                    tr {
                        th { "Name" }
                        td { "Beam B-101" }
                    }
                    tr {
                        th { "Material" }
                        td { "A992 Steel" }
                    }
                    tr {
                        th { "Section" }
                        td { "W12×58" }
                    }
                    tr {
                        th { "Length" }
                        td { "24 ft" }
                    }
                    tr {
                        th { "Module" }
                        td { "core-ui" }
                    }
                }
            }
        }
    }
}
