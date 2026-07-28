//! Shows whether a flow channel has an active value.

use crate::context::use_flow;
use dioxus::prelude::*;
use ses_shell::FlowChannelId;

#[component]
pub fn BindingIndicator(channel: String) -> Element {
    let flow = use_flow();
    let bound = flow
        .read()
        .get(&FlowChannelId::new(channel.clone()))
        .is_some();

    rsx! {
        span {
            class: if bound { "ses-binding-dot" } else { "ses-binding-dot ses-unbound" },
            title: if bound {
                format!("Bound: {channel}")
            } else {
                format!("Unbound: {channel}")
            },
        }
    }
}
