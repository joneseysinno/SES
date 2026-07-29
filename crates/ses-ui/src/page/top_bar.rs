//! Page-level sticky top bar (distinct from the global shell `TopBarPod`).

use crate::context::use_flow;
use crate::io::BindingIndicator;
use dioxus::prelude::*;
use ses_shell::{
    FlowChannelId, PageTopBar, TopBarAlign, TopBarSlot, TopBarSlotKind,
};

const OVERFLOW_THRESHOLD: usize = 8;

#[component]
pub fn PageTopBarView(bar: PageTopBar, on_action: EventHandler<String>) -> Element {
    if !bar.visible {
        return rsx! {};
    }

    let height = bar.height.px();
    let mut overflow_open = use_signal(|| false);

    let (left, center, right, overflow): (
        Vec<TopBarSlot>,
        Vec<TopBarSlot>,
        Vec<TopBarSlot>,
        Vec<TopBarSlot>,
    ) = {
        let mut left = Vec::new();
        let mut center = Vec::new();
        let mut right = Vec::new();
        for slot in bar.slots.iter().cloned() {
            match slot.align {
                TopBarAlign::Left => left.push(slot),
                TopBarAlign::Center => center.push(slot),
                TopBarAlign::Right => right.push(slot),
            }
        }
        let total = left.len() + center.len() + right.len();
        if total > OVERFLOW_THRESHOLD {
            let overflow = right.split_off(right.len().saturating_sub(total - OVERFLOW_THRESHOLD));
            (left, center, right, overflow)
        } else {
            (left, center, right, Vec::new())
        }
    };

    rsx! {
        div {
            class: "ses-page-top-bar",
            style: "height: {height}px; min-height: {height}px;",
            div { class: "ses-topbar-zone ses-topbar-zone-left",
                for slot in left {
                    TopBarSlotView { slot, on_action }
                }
            }
            div { class: "ses-topbar-zone ses-topbar-zone-center",
                for slot in center {
                    TopBarSlotView { slot, on_action }
                }
            }
            div { class: "ses-topbar-zone ses-topbar-zone-right",
                for slot in right {
                    TopBarSlotView { slot, on_action }
                }
                if !overflow.is_empty() {
                    div { class: "ses-topbar-overflow",
                        button {
                            class: "ses-ghost",
                            title: "More",
                            onclick: move |_| {
                                let next = !*overflow_open.peek();
                                overflow_open.set(next);
                            },
                            "…"
                        }
                        if overflow_open() {
                            div { class: "ses-topbar-overflow-menu",
                                for slot in overflow {
                                    TopBarSlotView { slot, on_action }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TopBarSlotView(slot: TopBarSlot, on_action: EventHandler<String>) -> Element {
    match slot.kind {
        TopBarSlotKind::Button { label, action_id } => {
            rsx! {
                button {
                    class: "ses-topbar-btn",
                    onclick: move |_| on_action.call(action_id.clone()),
                    "{label}"
                }
            }
        }
        TopBarSlotKind::Label { text } => {
            rsx! {
                span { class: "ses-topbar-label", "{text}" }
            }
        }
        TopBarSlotKind::FlowDisplay {
            channel,
            show_channel_name,
        } => {
            rsx! {
                FlowDisplaySlot { channel, show_channel_name }
            }
        }
        TopBarSlotKind::Separator => {
            rsx! {
                div { class: "ses-topbar-sep" }
            }
        }
    }
}

#[component]
fn FlowDisplaySlot(channel: String, show_channel_name: bool) -> Element {
    let flow = use_flow();
    let value_text = {
        let f = flow.read();
        f.get(&FlowChannelId::new(channel.clone()))
            .map(|v| v.display())
            .unwrap_or_else(|| "—".into())
    };

    rsx! {
        div { class: "ses-topbar-flow",
            BindingIndicator { channel: channel.clone() }
            div {
                span { "{value_text}" }
                if show_channel_name {
                    span { class: "ses-topbar-flow-channel", "{channel}" }
                }
            }
        }
    }
}
