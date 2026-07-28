//! Page leaf chrome — header, maximize, pod swap, body + I/O.

use crate::context::use_shell;
use crate::io::{InputContainer, OutputContainer};
use crate::pod::PodHost;
use dioxus::prelude::*;
use ses_shell::{
    Axis, IoPlacement, PageLeaf, PodKind,
    ops::{maximize_leaf, restore_layout, set_leaf_pod, split_leaf},
};

#[component]
pub fn PageLeafView(leaf: PageLeaf) -> Element {
    let mut shell = use_shell();
    let leaf_id = leaf.id;
    let kind = leaf.pod.kind;
    let module_id = leaf.pod.module_id.clone();
    let io = leaf.io.clone();
    let channel = io.channel.clone().unwrap_or_else(|| "calc.result".into());

    let is_maximized = {
        let s = shell.read();
        s.active()
            .map(|w| w.maximized == Some(leaf_id))
            .unwrap_or(false)
    };

    let body_class = if io.show_input || io.show_output {
        match io.placement {
            IoPlacement::Side => "ses-page-body ses-io-side",
            IoPlacement::Below => "ses-page-body ses-with-io",
        }
    } else {
        "ses-page-body"
    };

    rsx! {
        div { class: "ses-page-leaf",
            div { class: "ses-page-header",
                select {
                    value: "{kind.display_name()}",
                    onchange: move |evt| {
                        let label = evt.value();
                        let new_kind = PodKind::page_kinds()
                            .iter()
                            .copied()
                            .find(|k| k.display_name() == label)
                            .unwrap_or(PodKind::View);
                        let mod_id = match new_kind {
                            PodKind::Calculation => "analysis",
                            _ => "core-ui",
                        };
                        if let Some(ws) = shell.write().active_mut() {
                            set_leaf_pod(&mut ws.layout, leaf_id, new_kind, mod_id);
                        }
                    },
                    for k in PodKind::page_kinds() {
                        option {
                            selected: *k == kind,
                            value: "{k.display_name()}",
                            "{k.display_name()}"
                        }
                    }
                }
                span { class: "ses-muted", style: "font-size: 10px;",
                    "{module_id}"
                }
                div { class: "ses-page-header-actions",
                    button {
                        class: "ses-ghost",
                        title: "Split vertical (left/right)",
                        onclick: move |_| {
                            if let Some(ws) = shell.write().active_mut() {
                                split_leaf(
                                    &mut ws.layout,
                                    leaf_id,
                                    Axis::Horizontal,
                                    0.5,
                                    None,
                                );
                            }
                        },
                        "▥"
                    }
                    button {
                        class: "ses-ghost",
                        title: "Split horizontal (top/bottom)",
                        onclick: move |_| {
                            if let Some(ws) = shell.write().active_mut() {
                                split_leaf(
                                    &mut ws.layout,
                                    leaf_id,
                                    Axis::Vertical,
                                    0.5,
                                    None,
                                );
                            }
                        },
                        "▤"
                    }
                    button {
                        class: "ses-ghost",
                        title: if is_maximized { "Restore" } else { "Maximize" },
                        onclick: move |_| {
                            if let Some(ws) = shell.write().active_mut() {
                                if ws.maximized == Some(leaf_id) {
                                    restore_layout(ws);
                                } else {
                                    maximize_leaf(ws, leaf_id);
                                }
                            }
                        },
                        if is_maximized { "▾" } else { "▴" }
                    }
                }
            }
            div { class: "{body_class}",
                PodHost { kind, channel: channel.clone() }
                if io.show_input {
                    InputContainer { channel: channel.clone() }
                }
                if io.show_output {
                    OutputContainer { channel: channel.clone() }
                }
            }
        }
    }
}
