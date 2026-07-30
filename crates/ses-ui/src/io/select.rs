//! Select and multi-select fields.

use crate::io::field::FieldMeta;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectDef {
    pub meta: FieldMeta,
    pub value: String,
    pub options: Vec<SelectOption>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MultiSelectDef {
    pub meta: FieldMeta,
    pub values: Vec<String>,
    pub options: Vec<SelectOption>,
}

#[component]
pub fn SelectInput(
    meta: FieldMeta,
    value: String,
    options: Vec<SelectOption>,
    onchange: EventHandler<String>,
) -> Element {
    let field_class = if meta.error.is_some() {
        "ses-io-field ses-invalid"
    } else {
        "ses-io-field"
    };

    rsx! {
        div { class: "{field_class}",
            div { class: "ses-io-field-row",
                label { class: "ses-io-field-label", r#for: "{meta.id}", "{meta.label}" }
                select {
                    id: "{meta.id}",
                    class: "ses-io-field-control",
                    disabled: meta.disabled,
                    value: "{value}",
                    onchange: move |e| onchange.call(e.value()),
                    for opt in options.iter() {
                        option {
                            key: "{opt.value}",
                            value: "{opt.value}",
                            "{opt.label}"
                        }
                    }
                }
            }
            if let Some(err) = meta.error.clone() {
                div { class: "ses-io-field-error", "{err}" }
            }
        }
    }
}

#[component]
pub fn MultiSelectInput(
    meta: FieldMeta,
    values: Vec<String>,
    options: Vec<SelectOption>,
    onchange: EventHandler<Vec<String>>,
) -> Element {
    let field_class = if meta.error.is_some() {
        "ses-io-field ses-invalid"
    } else {
        "ses-io-field"
    };
    let selected: Vec<String> = values.clone();

    rsx! {
        div { class: "{field_class}",
            div { class: "ses-io-field-row",
                label { class: "ses-io-field-label", r#for: "{meta.id}", "{meta.label}" }
                select {
                    id: "{meta.id}",
                    class: "ses-io-field-control",
                    disabled: meta.disabled,
                    multiple: true,
                    onchange: move |e| {
                        let selected_values: Vec<String> = e
                            .values()
                            .into_iter()
                            .map(|(key, _)| key)
                            .collect();
                        onchange.call(selected_values);
                    },
                    for opt in options.iter() {
                        option {
                            key: "{opt.value}",
                            value: "{opt.value}",
                            selected: selected.contains(&opt.value),
                            "{opt.label}"
                        }
                    }
                }
            }
            if let Some(err) = meta.error.clone() {
                div { class: "ses-io-field-error", "{err}" }
            }
        }
    }
}
