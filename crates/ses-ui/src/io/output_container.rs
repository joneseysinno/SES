//! Output container — displays values from the flow bus.

use crate::context::use_flow;
use crate::io::binding_indicator::BindingIndicator;
use dioxus::prelude::*;
use ses_shell::FlowChannelId;

#[component]
pub fn OutputContainer(channel: String) -> Element {
    let flow = use_flow();
    let value = flow
        .read()
        .get(&FlowChannelId::new(channel.clone()))
        .map(|v| v.display());

    rsx! {
        div { class: "ses-io-panel",
            div { class: "ses-io-header",
                BindingIndicator { channel: channel.clone() }
                span { "Output" }
                span { class: "ses-muted", "{channel}" }
            }
            div { class: "ses-io-body",
                if let Some(text) = value {
                    pre { class: "ses-flow-value", "{text}" }
                } else {
                    p { class: "ses-muted", "No value on channel yet. Run a calculation or push an input." }
                }
            }
        }
    }
}
