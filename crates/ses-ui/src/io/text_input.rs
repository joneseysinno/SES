//! Text input field — label, value, optional units.

use dioxus::prelude::*;

#[component]
pub fn TextInput(
    id: String,
    label: String,
    value: String,
    oninput: EventHandler<String>,
    #[props(default)]
    units: Option<String>,
    #[props(default)]
    placeholder: Option<String>,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    error: Option<String>,
) -> Element {
    let field_class = if error.is_some() {
        "ses-io-field ses-invalid"
    } else {
        "ses-io-field"
    };
    let ph = placeholder.clone().unwrap_or_default();

    rsx! {
        div { class: "{field_class}",
            div { class: "ses-io-field-row",
                label { class: "ses-io-field-label", r#for: "{id}", "{label}" }
                input {
                    id: "{id}",
                    r#type: "text",
                    class: "ses-io-field-control",
                    value: "{value}",
                    placeholder: "{ph}",
                    disabled: disabled,
                    oninput: move |e| oninput.call(e.value()),
                }
                if let Some(u) = units.clone() {
                    span { class: "ses-io-field-units", "{u}" }
                }
            }
            if let Some(err) = error.clone() {
                div { class: "ses-io-field-error", "{err}" }
            }
        }
    }
}
