//! App menu ribbon — File / Edit / View / Modules / Help (placeholder commands).

use crate::context::{use_modules, use_shell, use_startup};
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
    let mut startup = use_startup();
    let modules = use_modules();
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
                                                    let s = shell.read();
                                                    let order: Vec<_> =
                                                        s.workspaces.iter().map(|w| w.id).collect();
                                                    let active = s.active_workspace;
                                                    drop(s);
                                                    let mut profile = startup.write();
                                                    profile.workspace_order = order;
                                                    profile.active_workspace = Some(active);
                                                    shell.write().status_message =
                                                        "Saved current layout as startup".into();
                                                    open.set(None);
                                                },
                                                "Save current layout as startup"
                                            }
                                            button {
                                                onclick: move |_| {
                                                    let mods = modules.read();
                                                    let key = shell
                                                        .read()
                                                        .active()
                                                        .and_then(|w| w.seed_key.clone());
                                                    if let Some(key) = key {
                                                        let factory = ses_shell::factory_workspaces(
                                                            &[],
                                                            mods.logical.all_factory_workspaces(),
                                                        );
                                                        if let Some(seed) =
                                                            factory.into_iter().find(|w| {
                                                                w.seed_key.as_deref() == Some(key.as_str())
                                                            })
                                                        {
                                                            let mut s = shell.write();
                                                            if let Some(ws) = s.active_mut() {
                                                                let id = ws.id;
                                                                let name = ws.name.clone();
                                                                *ws = seed;
                                                                ws.id = id;
                                                                ws.name = name;
                                                                ws.user_modified = false;
                                                            }
                                                            s.status_message =
                                                                "Workspace reset to factory".into();
                                                        }
                                                    } else {
                                                        shell.write().status_message =
                                                            "No factory seed for this workspace".into();
                                                    }
                                                    open.set(None);
                                                },
                                                "Reset this workspace to factory"
                                            }
                                            button {
                                                onclick: move |_| {
                                                    *startup.write() =
                                                        ses_shell::StartupProfile::default();
                                                    let mods = modules.read();
                                                    let factory = ses_shell::factory_workspaces(
                                                        &[],
                                                        mods.logical.all_factory_workspaces(),
                                                    );
                                                    let state = ses_shell::resolve_startup(
                                                        factory,
                                                        Vec::new(),
                                                        &ses_shell::StartupProfile::default(),
                                                    );
                                                    *shell.write() = state;
                                                    shell.write().status_message =
                                                        "Reset all to factory".into();
                                                    open.set(None);
                                                },
                                                "Reset all to factory"
                                            }
                                            button {
                                                onclick: move |_| {
                                                    let mut profile = startup.write();
                                                    profile.factory_seed_disabled =
                                                        !profile.factory_seed_disabled;
                                                    let on = profile.factory_seed_disabled;
                                                    drop(profile);
                                                    shell.write().status_message = if on {
                                                        "Factory reseeding disabled".into()
                                                    } else {
                                                        "Factory reseeding enabled".into()
                                                    };
                                                    open.set(None);
                                                },
                                                "Never reseed factory workspaces"
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
