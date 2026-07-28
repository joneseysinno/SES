//! App menu ribbon — File / Edit / View / Modules / Help (placeholder commands).

use crate::context::use_shell;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RibbonMenu {
    File,
    Edit,
    View,
    Modules,
    Help,
}

impl RibbonMenu {
    fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Modules => "Modules",
            Self::Help => "Help",
        }
    }

    fn all() -> &'static [RibbonMenu] {
        &[
            Self::File,
            Self::Edit,
            Self::View,
            Self::Modules,
            Self::Help,
        ]
    }
}

#[component]
pub fn AppRibbon() -> Element {
    let mut shell = use_shell();
    let mut open = use_signal(|| None::<RibbonMenu>);

    rsx! {
        div {
            class: "ses-app-ribbon",
            for menu in RibbonMenu::all().iter().copied() {
                {
                    let is_open = open() == Some(menu);
                    rsx! {
                        div { class: "ses-app-ribbon-item",
                            button {
                                class: if is_open { "ses-app-ribbon-btn ses-open" } else { "ses-app-ribbon-btn" },
                                onclick: move |_| {
                                    if open() == Some(menu) {
                                        open.set(None);
                                    } else {
                                        open.set(Some(menu));
                                    }
                                },
                                "{menu.label()}"
                            }
                            if is_open {
                                div { class: "ses-app-ribbon-menu",
                                    match menu {
                                        RibbonMenu::File => rsx! {
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "File → New (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "New"
                                            }
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "File → Open (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Open…"
                                            }
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "File → Save (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Save"
                                            }
                                            div { class: "ses-menu-sep" }
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "File → Exit (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Exit"
                                            }
                                        },
                                        RibbonMenu::Edit => rsx! {
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "Edit → Undo (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Undo"
                                            }
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "Edit → Redo (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Redo"
                                            }
                                        },
                                        RibbonMenu::View => rsx! {
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "View → Reset layout (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Reset Layout"
                                            }
                                        },
                                        RibbonMenu::Modules => rsx! {
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "Modules → Manage (placeholder)".into();
                                                    open.set(None);
                                                },
                                                "Manage Modules…"
                                            }
                                        },
                                        RibbonMenu::Help => rsx! {
                                            button {
                                                onclick: move |_| {
                                                    shell.write().status_message = "Help → About SES".into();
                                                    open.set(None);
                                                },
                                                "About SES"
                                            }
                                        },
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
