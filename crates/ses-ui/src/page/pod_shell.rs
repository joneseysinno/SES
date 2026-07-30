//! Pod chrome — header, collapse toggle, landmark hook.

use dioxus::prelude::*;
use ses_shell::{PodDescriptor, PodKind};

#[component]
pub fn PodShell(
    pod: PodDescriptor,
    children: Element,
    on_toggle: Option<EventHandler<bool>>,
) -> Element {
    let collapsible = pod.kind.collapsible();
    let collapsed = pod.collapsed && collapsible;
    let title = pod.title.clone();
    let kind_class = match pod.kind {
        PodKind::Summary => "ses-pod-shell ses-pod-summary",
        PodKind::Anchor => "ses-pod-shell ses-pod-anchor",
        PodKind::Scroller => "ses-pod-shell ses-pod-scroller",
        PodKind::Section => "ses-pod-shell ses-pod-section",
        PodKind::Custom => "ses-pod-shell ses-pod-custom",
    };
    let shell_class = if collapsed {
        format!("{kind_class} ses-pod-collapsed")
    } else {
        kind_class.to_string()
    };

    rsx! {
        section {
            class: "{shell_class}",
            id: "pod-{pod.id}",
            header { class: "ses-pod-header",
                if let Some(label) = pod.landmark_label.clone() {
                    span { class: "ses-pod-landmark-flag", title: "Landmark", "{label}" }
                }
                span { class: "ses-pod-title", "{title}" }
                if collapsible {
                    button {
                        class: "ses-ghost",
                        title: if collapsed { "Expand" } else { "Collapse" },
                        onclick: move |_| {
                            if let Some(handler) = on_toggle {
                                handler.call(!collapsed);
                            }
                        },
                        if collapsed { "▶" } else { "▼" }
                    }
                }
            }
            if !collapsed {
                div { class: "ses-pod-body",
                    {children}
                }
            }
        }
    }
}
