//! Numerical input field — label, numeric value, units, parse feedback.

use dioxus::prelude::*;

fn numerical_error(value: &str, external: Option<&str>) -> Option<String> {
    if let Some(err) = external {
        return Some(err.to_string());
    }
    if value.trim().is_empty() {
        return None;
    }
    if value.trim().parse::<f64>().is_err() {
        Some("Enter a valid number".into())
    } else {
        None
    }
}

#[component]
pub fn NumericalInput(
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
    let parsed_error = numerical_error(&value, error.as_deref());
    let field_class = if parsed_error.is_some() {
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
                    inputmode: "decimal",
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
            if let Some(err) = parsed_error {
                div { class: "ses-io-field-error", "{err}" }
            }
        }
    }
}
