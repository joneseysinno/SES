//! Input container — parameter forms bound to a flow channel.

use crate::context::use_flow;
use crate::io::binding_indicator::BindingIndicator;
use dioxus::prelude::*;
use ses_shell::{FlowChannelId, FlowValue};

#[component]
pub fn InputContainer(channel: String) -> Element {
    let mut flow = use_flow();
    let mut text = use_signal(|| String::new());
    let ch = channel.clone();

    // Seed from existing value
    {
        let existing = flow
            .read()
            .get(&FlowChannelId::new(channel.clone()))
            .map(|v| v.display());
        if let Some(v) = existing {
            if text.read().is_empty() {
                text.set(v);
            }
        }
    }

    rsx! {
        div { class: "ses-io-panel",
            div { class: "ses-io-header",
                BindingIndicator { channel: channel.clone() }
                span { "Input" }
                span { class: "ses-muted", "{channel}" }
            }
            div { class: "ses-io-body",
                label {
                    style: "display: flex; flex-direction: column; gap: 4px; font-size: 11px; color: var(--ses-text-dim);",
                    "Override / seed value"
                    input {
                        value: "{text}",
                        oninput: move |e| text.set(e.value()),
                    }
                }
                button {
                    style: "margin-top: 8px;",
                    onclick: move |_| {
                        let val = text.read().clone();
                        if let Ok(n) = val.parse::<f64>() {
                            flow.write().publish(ch.as_str(), FlowValue::Number(n));
                        } else {
                            flow.write().publish(ch.as_str(), FlowValue::Text(val));
                        }
                    },
                    "Push to flow"
                }
            }
        }
    }
}
