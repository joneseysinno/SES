//! Calculation pod — writes results to the flow bus.

use crate::context::{use_flow, use_shell};
use dioxus::prelude::*;
use ses_shell::FlowValue;

#[component]
pub fn CalculationPod(channel: String) -> Element {
    let mut flow = use_flow();
    let mut shell = use_shell();
    let mut a = use_signal(|| "10".to_string());
    let mut b = use_signal(|| "20".to_string());
    let ch = channel.clone();

    rsx! {
        div { class: "ses-pod",
            div { class: "ses-pod-title", "Calculation" }
            div { class: "ses-calc-form",
                label {
                    "Input A"
                    input {
                        r#type: "text",
                        value: "{a}",
                        oninput: move |e| a.set(e.value()),
                    }
                }
                label {
                    "Input B"
                    input {
                        r#type: "text",
                        value: "{b}",
                        oninput: move |e| b.set(e.value()),
                    }
                }
                div { class: "ses-row",
                    button {
                        class: "ses-active",
                        onclick: move |_| {
                            let av = a.read().parse::<f64>().unwrap_or(0.0);
                            let bv = b.read().parse::<f64>().unwrap_or(0.0);
                            let sum = av + bv;
                            flow.write().publish(
                                ch.as_str(),
                                FlowValue::Json(serde_json::json!({
                                    "op": "add",
                                    "a": av,
                                    "b": bv,
                                    "result": sum,
                                })),
                            );
                            shell.write().status_message =
                                format!("Published {ch} = {sum}");
                        },
                        "Run"
                    }
                    span { class: "ses-muted", "channel: {channel}" }
                }
            }
        }
    }
}
