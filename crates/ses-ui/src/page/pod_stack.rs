//! Stack / grid layout host for pods within a page.

use crate::page::pod_shell::PodShell;
use dioxus::prelude::*;
use ses_shell::{
    PodDescriptor, PodLayout,
    ops::{effective_pod_layout, ordered_pods},
};

/// Renders pod chrome in stack or grid order. Callers supply body content
/// per pod via the `bodies` map (keyed by pod id string).
#[component]
pub fn PodStack(
    pods: Vec<PodDescriptor>,
    layout: PodLayout,
    viewport_px: u32,
    #[props(default)] bodies: Vec<(u64, Element)>,
) -> Element {
    let effective = effective_pod_layout(&layout, viewport_px);
    let ordered: Vec<PodDescriptor> = ordered_pods(&pods).into_iter().cloned().collect();
    let body_map: std::collections::HashMap<u64, Element> = bodies.into_iter().collect();

    let cells = ordered.into_iter().map(|pod| {
        let id = pod.id.0;
        let body = body_map.get(&id).cloned().unwrap_or_else(|| {
            rsx! { div { class: "ses-muted", "Empty pod" } }
        });
        (pod, body)
    });

    match effective {
        PodLayout::Stack => {
            rsx! {
                div { class: "ses-pod-stack",
                    for (pod, body) in cells {
                        PodShell { pod,
                            {body}
                        }
                    }
                }
            }
        }
        PodLayout::Grid { cols, .. } => {
            rsx! {
                div {
                    class: "ses-pod-grid",
                    style: "--cols: {cols};",
                    for (pod, body) in cells {
                        {
                            let span = pod.col_span;
                            rsx! {
                                div {
                                    class: "ses-pod-grid-cell",
                                    style: "grid-column: span {span};",
                                    PodShell { pod,
                                        {body}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
