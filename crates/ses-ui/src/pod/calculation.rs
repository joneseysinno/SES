//! Calculation pod — writes results to the flow bus.

use crate::context::{use_flow, use_shell};
use crate::io::{EngineerInfo, EngineerInput, NumericalInput, TextInput};
use dioxus::prelude::*;
use ses_shell::FlowValue;

#[component]
pub fn CalculationPod(channel: String) -> Element {
    let mut flow = use_flow();
    let mut shell = use_shell();
    let mut a = use_signal(|| "10".to_string());
    let mut b = use_signal(|| "20".to_string());
    let mut scale = use_signal(|| "1".to_string());
    let mut note = use_signal(|| String::new());
    let ch = channel.clone();

    let info_a = EngineerInfo::new()
        .with_description("Primary addend for the demo calculation.")
        .with_code_ref("ACI 318 §R5.1 (placeholder)")
        .with_validation_note("Must be a finite number.");

    let info_b = EngineerInfo::new()
        .with_description("Secondary addend for the demo calculation.")
        .with_code_ref("Internal: CalculationPod::run")
        .with_validation_note("Must be a finite number.");

    rsx! {
        div { class: "ses-pod",
            div { class: "ses-pod-fields",
                EngineerInput {
                    id: "calc-a",
                    label: "Input A",
                    value: a(),
                    units: Some("—".into()),
                    info: info_a,
                    oninput: move |v| a.set(v),
                }
                EngineerInput {
                    id: "calc-b",
                    label: "Input B",
                    value: b(),
                    units: Some("—".into()),
                    info: info_b,
                    oninput: move |v| b.set(v),
                }
                NumericalInput {
                    id: "calc-scale",
                    label: "Scale",
                    value: scale(),
                    units: Some("×".into()),
                    oninput: move |v| scale.set(v),
                }
                TextInput {
                    id: "calc-note",
                    label: "Note",
                    value: note(),
                    placeholder: Some("Optional remark".into()),
                    oninput: move |v| note.set(v),
                }
                div { class: "ses-row",
                    button {
                        class: "ses-active",
                        onclick: move |_| {
                            let av = a.read().parse::<f64>().unwrap_or(0.0);
                            let bv = b.read().parse::<f64>().unwrap_or(0.0);
                            let sv = scale.read().parse::<f64>().unwrap_or(1.0);
                            let sum = (av + bv) * sv;
                            let remark = note.read().clone();
                            flow.write().publish(
                                ch.as_str(),
                                FlowValue::Json(serde_json::json!({
                                    "op": "add",
                                    "a": av,
                                    "b": bv,
                                    "scale": sv,
                                    "result": sum,
                                    "note": remark,
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
