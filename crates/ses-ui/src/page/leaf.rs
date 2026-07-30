//! Page leaf chrome — header, maximize, page swap, body + I/O, landmark menu.

use crate::context::{use_modules_ui, use_shell, use_user};
use crate::io::{InputContainer, OutputContainer};
use crate::page::page_area::{
    LandmarkGroupCtx, LandmarkGroupDraft, LeafMountCtx, register_leaf_mount,
};
use crate::page::page_view::PageView;
use dioxus::prelude::*;
use ses_shell::{
    Axis, IoPlacement, LandmarkAnchor, LandmarkDef, LandmarkIcon, LandmarkId, PageDescriptor,
    PageLeaf, PodId,
    ops::{
        add_landmark, maximize_leaf, remove_landmark, restore_layout, set_leaf_page, split_leaf,
    },
};

const COLOR_SWATCHES: &[&str] = &[
    "var(--ses-accent)",
    "#c25555",
    "#55c27a",
    "#c2a055",
    "#9b59b6",
];

#[derive(Clone, PartialEq)]
enum MenuMode {
    Closed,
    Menu,
    Form { edit_id: Option<LandmarkId> },
}

#[component]
pub fn PageLeafView(leaf: PageLeaf) -> Element {
    let mut shell = use_shell();
    let modules = use_modules_ui();
    let user = use_user();
    let leaf_id = leaf.id;
    let page = leaf.page.clone();
    let module_id = page.module_id.clone();
    let page_title = page.title.clone();
    let display_title = page.display_title().to_string();
    let io = leaf.io.clone();
    let channel = io.channel.clone().unwrap_or_else(|| "calc.result".into());

    let mut menu = use_signal(|| MenuMode::Closed);
    let mut form_label = use_signal(|| "★".to_string());
    let mut form_tooltip = use_signal(String::new);
    let mut form_color = use_signal(|| None::<String>);

    let leaf_mounts = try_use_context::<LeafMountCtx>();
    let group_draft = try_use_context::<LandmarkGroupCtx>();

    let is_maximized = {
        let s = shell.read();
        s.active()
            .map(|w| w.maximized == Some(leaf_id))
            .unwrap_or(false)
    };

    let existing = {
        let s = shell.read();
        s.active().and_then(|w| {
            w.landmarks
                .iter()
                .find(|lm| lm.leaf_ids().contains(&leaf_id))
                .cloned()
        })
    };

    let selecting = group_draft
        .as_ref()
        .and_then(|g| g.read().as_ref().map(|d| d.selected.contains(&leaf_id)))
        .unwrap_or(false);
    let in_select_mode = group_draft
        .as_ref()
        .map(|g| g.read().is_some())
        .unwrap_or(false);

    let leaf_class = {
        let mut c = "ses-page-leaf".to_string();
        if in_select_mode {
            c.push_str(" ses-landmark-selecting");
        }
        if selecting {
            c.push_str(" ses-landmark-selected");
        }
        c
    };

    let body_class = if io.show_input || io.show_output {
        match io.placement {
            IoPlacement::Side => "ses-page-body ses-io-side",
            IoPlacement::Below => "ses-page-body ses-with-io",
        }
    } else {
        "ses-page-body"
    };

    let picker_pages: Vec<(String, String, String)> = {
        let s = shell.read();
        let u = user.read();
        let mods = modules.read();
        s.active()
            .map(|ws| {
                mods.logical
                    .pages_for_workspace(ws, &u)
                    .into_iter()
                    .map(|(mid, p)| {
                        (
                            mid.0,
                            p.page_id.0.clone(),
                            p.display_name.to_string(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    let mut open_form = move |edit: Option<LandmarkDef>| {
        if let Some(lm) = edit {
            form_label.set(lm.icon.label.clone());
            form_tooltip.set(lm.tooltip.clone().unwrap_or_default());
            form_color.set(lm.icon.color.clone());
            menu.set(MenuMode::Form {
                edit_id: Some(lm.id),
            });
        } else {
            form_label.set("★".into());
            form_tooltip.set(String::new());
            form_color.set(None);
            menu.set(MenuMode::Form { edit_id: None });
        }
    };

    let existing_id = existing.as_ref().map(|lm| lm.id);
    let existing_for_edit = existing.clone();
    let current_key = format!("{}:{}", page.module_id, page.page_id);

    rsx! {
        div {
            class: "{leaf_class}",
            onmounted: move |e| {
                if let Some(mounts) = leaf_mounts {
                    register_leaf_mount(mounts, leaf_id, e.data());
                }
            },
            div {
                class: "ses-page-header",
                onclick: move |_| {
                    if let Some(mut draft) = group_draft {
                        let mut g = draft.write();
                        if let Some(d) = g.as_mut() {
                            if let Some(pos) = d.selected.iter().position(|id| *id == leaf_id) {
                                d.selected.remove(pos);
                            } else {
                                d.selected.push(leaf_id);
                            }
                        }
                    }
                },
                select {
                    value: "{current_key}",
                    onchange: move |evt| {
                        let key = evt.value();
                        let Some((mod_id, page_id)) = key.split_once(':') else {
                            return;
                        };
                        let mut desc = PageDescriptor::new(mod_id, page_id);
                        // Preserve title override when swapping within same module.
                        if let Some(t) = page_title.clone() {
                            desc = desc.with_title(t);
                        }
                        if let Some(ws) = shell.write().active_mut() {
                            set_leaf_page(&mut ws.layout, leaf_id, desc);
                        }
                    },
                    for (mod_id, page_id, display_name) in picker_pages.iter() {
                        {
                            let key = format!("{mod_id}:{page_id}");
                            let label = format!("{mod_id} — {display_name}");
                            let selected = key == current_key;
                            rsx! {
                                option {
                                    selected: selected,
                                    value: "{key}",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
                span { class: "ses-page-header-title", "{display_title}" }
                span { class: "ses-muted", style: "font-size: 10px;",
                    "{module_id}"
                }
                div { class: "ses-page-header-actions",
                    button {
                        class: "ses-ghost",
                        title: "Landmark",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            let next = match *menu.peek() {
                                MenuMode::Closed => MenuMode::Menu,
                                _ => MenuMode::Closed,
                            };
                            menu.set(next);
                        },
                        "⚑"
                    }
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

                match menu() {
                    MenuMode::Menu => {
                        rsx! {
                            div {
                                class: "ses-landmark-menu",
                                onpointerdown: move |evt| evt.stop_propagation(),
                                if let Some(lm) = existing_for_edit.clone() {
                                    button {
                                        onclick: move |_| open_form(Some(lm.clone())),
                                        "Edit landmark"
                                    }
                                    button {
                                        class: "ses-danger-action",
                                        onclick: move |_| {
                                            if let Some(id) = existing_id {
                                                let mut s = shell.write();
                                                if let Some(ws) = s.active_mut() {
                                                    remove_landmark(ws, id);
                                                }
                                                s.status_message = "Landmark removed".into();
                                            }
                                            menu.set(MenuMode::Closed);
                                        },
                                        "Remove landmark"
                                    }
                                } else {
                                    button {
                                        onclick: move |_| open_form(None),
                                        "Add landmark here"
                                    }
                                }
                                button {
                                    onclick: move |_| {
                                        if let Some(mut draft) = group_draft {
                                            draft.set(Some(LandmarkGroupDraft {
                                                selected: vec![leaf_id],
                                            }));
                                        }
                                        shell.write().status_message =
                                            "Select pods, then confirm group from any ⚑ menu".into();
                                        menu.set(MenuMode::Closed);
                                    },
                                    "Select pods to group…"
                                }
                                if in_select_mode {
                                    button {
                                        onclick: move |_| {
                                            form_label.set("GRP".into());
                                            form_tooltip.set(String::new());
                                            form_color.set(None);
                                            menu.set(MenuMode::Form { edit_id: None });
                                        },
                                        "Confirm group…"
                                    }
                                    button {
                                        onclick: move |_| {
                                            if let Some(mut draft) = group_draft {
                                                draft.set(None);
                                            }
                                            menu.set(MenuMode::Closed);
                                        },
                                        "Cancel grouping"
                                    }
                                }
                            }
                        }
                    }
                    MenuMode::Form { edit_id } => {
                        rsx! {
                            div {
                                class: "ses-landmark-menu",
                                onpointerdown: move |evt| evt.stop_propagation(),
                                div { class: "ses-landmark-form",
                                    label {
                                        "Label (max 3)"
                                        input {
                                            value: "{form_label}",
                                            maxlength: "3",
                                            oninput: move |evt| {
                                                let mut v = evt.value();
                                                v.truncate(3);
                                                form_label.set(v);
                                            },
                                        }
                                    }
                                    label {
                                        "Tooltip"
                                        input {
                                            value: "{form_tooltip}",
                                            oninput: move |evt| form_tooltip.set(evt.value()),
                                        }
                                    }
                                    div { class: "ses-landmark-swatches",
                                        for swatch in COLOR_SWATCHES {
                                            {
                                                let sw = (*swatch).to_string();
                                                let active = form_color()
                                                    .as_ref()
                                                    .map(|c| c == swatch)
                                                    .unwrap_or(false);
                                                rsx! {
                                                    button {
                                                        class: if active {
                                                            "ses-landmark-swatch ses-active"
                                                        } else {
                                                            "ses-landmark-swatch"
                                                        },
                                                        style: "background: {sw};",
                                                        title: "{sw}",
                                                        onclick: move |_| {
                                                            form_color.set(Some(sw.clone()));
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    button {
                                        onclick: move |_| {
                                            let label = form_label.peek().clone();
                                            if label.is_empty() {
                                                return;
                                            }
                                            let mut icon = LandmarkIcon::new(label);
                                            if let Some(c) = form_color.peek().clone() {
                                                icon = icon.with_color(c);
                                            }
                                            let tip = form_tooltip.peek().clone();
                                            let tip = if tip.is_empty() { None } else { Some(tip) };
                                            // Whole-leaf landmark: sentinel pod id 0.
                                            let pod_id = PodId(0);

                                            if in_select_mode && edit_id.is_none() {
                                                if let Some(mut draft) = group_draft {
                                                    let selected = draft
                                                        .peek()
                                                        .as_ref()
                                                        .map(|d| d.selected.clone())
                                                        .unwrap_or_default();
                                                    if selected.len() >= 2 {
                                                        let anchors: Vec<_> = selected
                                                            .into_iter()
                                                            .map(|lid| LandmarkAnchor::new(lid, PodId(0)))
                                                            .collect();
                                                        let mut s = shell.write();
                                                        if let Some(ws) = s.active_mut() {
                                                            let mut lm =
                                                                LandmarkDef::group(anchors, icon);
                                                            lm.tooltip = tip;
                                                            ws.add_landmark(lm);
                                                        }
                                                        s.status_message =
                                                            "Group landmark added".into();
                                                        draft.set(None);
                                                    } else {
                                                        shell.write().status_message =
                                                            "Select at least 2 pods".into();
                                                        return;
                                                    }
                                                }
                                            } else if let Some(id) = edit_id {
                                                let mut s = shell.write();
                                                if let Some(ws) = s.active_mut() {
                                                    if let Some(lm) =
                                                        ws.landmarks.iter_mut().find(|lm| lm.id == id)
                                                    {
                                                        lm.icon = icon;
                                                        lm.tooltip = tip;
                                                    }
                                                }
                                                s.status_message = "Landmark updated".into();
                                            } else {
                                                let mut s = shell.write();
                                                if let Some(ws) = s.active_mut() {
                                                    let id = add_landmark(ws, leaf_id, pod_id, icon);
                                                    if let Some(lm) =
                                                        ws.landmarks.iter_mut().find(|lm| lm.id == id)
                                                    {
                                                        lm.tooltip = tip;
                                                    }
                                                }
                                                s.status_message = "Landmark added".into();
                                            }
                                            menu.set(MenuMode::Closed);
                                        },
                                        "Save"
                                    }
                                    button {
                                        class: "ses-ghost",
                                        onclick: move |_| menu.set(MenuMode::Closed),
                                        "Cancel"
                                    }
                                }
                            }
                        }
                    }
                    MenuMode::Closed => rsx! {}
                }
            }
            div { class: "{body_class}",
                PageView { page: page.clone(), leaf_id }
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
