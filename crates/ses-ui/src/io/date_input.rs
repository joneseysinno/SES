//! Date input field — label and ISO date value.

use crate::io::field::FieldMeta;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct DateDef {
    pub meta: FieldMeta,
    pub value: String,
}

#[component]
pub fn DateInput(
    meta: FieldMeta,
    value: String,
    oninput: EventHandler<String>,
) -> Element {
    let field_class = if meta.error.is_some() {
        "ses-io-field ses-invalid"
    } else {
        "ses-io-field"
    };
    let ph = meta.placeholder.clone().unwrap_or_default();

    rsx! {
        div { class: "{field_class}",
            div { class: "ses-io-field-row",
                label { class: "ses-io-field-label", r#for: "{meta.id}", "{meta.label}" }
                input {
                    id: "{meta.id}",
                    r#type: "date",
                    class: "ses-io-field-control",
                    value: "{value}",
                    placeholder: "{ph}",
                    disabled: meta.disabled,
                    oninput: move |e| oninput.call(e.value()),
                }
            }
            if let Some(err) = meta.error.clone() {
                div { class: "ses-io-field-error", "{err}" }
            }
        }
    }
}
