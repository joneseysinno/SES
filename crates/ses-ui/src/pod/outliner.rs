//! Outliner — dummy project tree.

use dioxus::prelude::*;

#[component]
pub fn OutlinerPod() -> Element {
    let mut selected = use_signal(|| "project".to_string());

    rsx! {
        div { class: "ses-pod",
            div { class: "ses-pod-title", "Outliner" }
            ul { class: "ses-outliner-tree",
                li {
                    div {
                        class: if selected() == "project" { "ses-outliner-item ses-selected" } else { "ses-outliner-item" },
                        onclick: move |_| selected.set("project".into()),
                        "📁 Sample Project"
                    }
                    ul {
                        li {
                            div {
                                class: if selected() == "model" { "ses-outliner-item ses-selected" } else { "ses-outliner-item" },
                                onclick: move |_| selected.set("model".into()),
                                "📐 Structural Model"
                            }
                            ul {
                                li {
                                    div {
                                        class: if selected() == "beam" { "ses-outliner-item ses-selected" } else { "ses-outliner-item" },
                                        onclick: move |_| selected.set("beam".into()),
                                        "Beam B-101"
                                    }
                                }
                                li {
                                    div {
                                        class: if selected() == "col" { "ses-outliner-item ses-selected" } else { "ses-outliner-item" },
                                        onclick: move |_| selected.set("col".into()),
                                        "Column C-12"
                                    }
                                }
                            }
                        }
                        li {
                            div {
                                class: if selected() == "loads" { "ses-outliner-item ses-selected" } else { "ses-outliner-item" },
                                onclick: move |_| selected.set("loads".into()),
                                "⬇ Load Cases"
                            }
                        }
                    }
                }
            }
        }
    }
}
