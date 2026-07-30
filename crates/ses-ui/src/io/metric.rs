//! Metric readout — large value with optional delta indicator.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MetricDef {
    pub label: String,
    pub value: String,
    pub delta: Option<String>,
    pub delta_up: Option<bool>,
}

#[component]
pub fn Metric(def: MetricDef) -> Element {
    let delta_class = match def.delta_up {
        Some(true) => "ses-metric-delta ses-metric-delta-up",
        Some(false) => "ses-metric-delta ses-metric-delta-down",
        None => "ses-metric-delta",
    };

    rsx! {
        div { class: "ses-metric",
            div { class: "ses-metric-value", "{def.value}" }
            div { class: "ses-metric-label", "{def.label}" }
            if let Some(delta) = def.delta.clone() {
                div { class: "{delta_class}", "{delta}" }
            }
        }
    }
}
