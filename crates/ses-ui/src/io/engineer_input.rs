//! Engineer input — card with label, value, units, and expandable info.

use crate::io::field::EngineerInfo;
use dioxus::prelude::*;

fn numerical_hint(value: &str, external: Option<&str>) -> Option<String> {
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
pub fn EngineerInput(
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
    #[props(default)]
    info: EngineerInfo,
) -> Element {
    let mut info_open = use_signal(|| false);
    let has_info = !info.is_empty();
    let parsed_error = numerical_hint(&value, error.as_deref());

    let mut card_class = "ses-io-engineer ses-io-field".to_string();
    if info_open() {
        card_class.push_str(" ses-expanded");
    }
    if parsed_error.is_some() {
        card_class.push_str(" ses-invalid");
    }

    let ph = placeholder.clone().unwrap_or_default();
    let info_btn_class = if info_open() {
        "ses-ghost ses-io-info-btn ses-active"
    } else {
        "ses-ghost ses-io-info-btn"
    };

    rsx! {
        div { class: "{card_class}",
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
                if has_info {
                    button {
                        class: "{info_btn_class}",
                        r#type: "button",
                        title: if info_open() { "Hide info" } else { "Show info" },
                        onclick: move |_| {
                            let next = !*info_open.peek();
                            info_open.set(next);
                        },
                        "ⓘ"
                    }
                }
            }
            if let Some(err) = parsed_error {
                div { class: "ses-io-field-error", "{err}" }
            }
            if info_open() && has_info {
                div { class: "ses-io-engineer-info",
                    if let Some(desc) = info.description.clone() {
                        p { class: "ses-io-engineer-desc", "{desc}" }
                    }
                    if !info.code_refs.is_empty() {
                        div { class: "ses-io-engineer-section",
                            span { class: "ses-io-engineer-section-title", "Code references" }
                            ul {
                                for r in info.code_refs.clone() {
                                    li { "{r}" }
                                }
                            }
                        }
                    }
                    if !info.validation_notes.is_empty() {
                        div { class: "ses-io-engineer-section",
                            span { class: "ses-io-engineer-section-title", "Validation" }
                            ul {
                                for n in info.validation_notes.clone() {
                                    li { "{n}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
