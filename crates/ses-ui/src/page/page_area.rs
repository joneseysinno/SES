//! Page area — sticky top bar, scrollable pod region, landmark scroll bar.

use crate::context::use_shell;
use crate::page::node::PageNodeView;
use crate::page::scroll_bar::LandmarkScrollBar;
use crate::page::top_bar::PageTopBarView;
use dioxus::prelude::*;
use ses_shell::{
    LandmarkDef, LeafId, PageNode, PageTopBar,
    ops::{maximize_leaf, scroll_fraction as calc_scroll_fraction},
};
use std::collections::HashMap;
use std::rc::Rc;

/// Registered leaf mount handles for position measurement / scroll-into-view.
pub type LeafMountMap = HashMap<LeafId, Rc<MountedData>>;

/// Multi-select state while building a group landmark from leaf headers.
#[derive(Clone, PartialEq)]
pub struct LandmarkGroupDraft {
    pub selected: Vec<LeafId>,
}

pub type LeafMountCtx = Signal<LeafMountMap>;
pub type LandmarkGroupCtx = Signal<Option<LandmarkGroupDraft>>;

#[component]
pub fn PageArea(
    node: PageNode,
    top_bar: Option<PageTopBar>,
    landmarks: Vec<LandmarkDef>,
) -> Element {
    let mut shell = use_shell();
    let mut scroll_frac = use_signal(|| 0.0f32);
    let mut thumb_ratio = use_signal(|| 1.0f32);
    let mut leaf_positions = use_signal(HashMap::<LeafId, f32>::new);
    let mut scroll_mounted = use_signal(|| None::<Rc<MountedData>>);
    let leaf_mounts: LeafMountCtx = use_signal(LeafMountMap::new);
    let group_draft: LandmarkGroupCtx = use_signal(|| None::<LandmarkGroupDraft>);

    use_context_provider(|| leaf_mounts);
    use_context_provider(|| group_draft);

    let landmarks_for_keys = landmarks.clone();

    let measure_leaves = move || {
        let mounts = leaf_mounts();
        let scroll = scroll_mounted();
        spawn(async move {
            let Some(scroll_data) = scroll else {
                return;
            };
            let Ok(scroll_rect) = scroll_data.get_client_rect().await else {
                return;
            };
            let scroll_offset = scroll_data
                .get_scroll_offset()
                .await
                .map(|v| v.y)
                .unwrap_or(0.0);
            let scroll_size = scroll_data
                .get_scroll_size()
                .await
                .map(|s| s.height)
                .unwrap_or(scroll_rect.height())
                .max(1.0);

            let mut next = HashMap::new();
            for (id, data) in mounts {
                if let Ok(rect) = data.get_client_rect().await {
                    let top_in_content =
                        rect.min_y() - scroll_rect.min_y() + scroll_offset;
                    let frac = (top_in_content / scroll_size).clamp(0.0, 1.0) as f32;
                    next.insert(id, frac);
                }
            }
            leaf_positions.set(next);
        });
    };

    // Re-measure when landmarks or layout identity changes.
    let _ = node.clone();
    let landmark_count = landmarks.len();
    use_effect(move || {
        let _ = landmark_count;
        measure_leaves();
    });

    let jump_to = move |frac: f32| {
        let Some(data) = scroll_mounted() else {
            return;
        };
        spawn(async move {
            let size = data
                .get_scroll_size()
                .await
                .map(|s| s.height)
                .unwrap_or(1.0);
            let client = data
                .get_client_rect()
                .await
                .map(|r| r.height())
                .unwrap_or(1.0);
            let range = (size - client).max(0.0);
            let y = (frac.clamp(0.0, 1.0) as f64) * range;
            let _ = data
                .scroll(
                    dioxus::html::geometry::PixelsVector2D::new(0.0, y),
                    dioxus::html::ScrollBehavior::Smooth,
                )
                .await;
            scroll_frac.set(frac.clamp(0.0, 1.0));
        });
    };

    let on_landmark_jump = move |frac: f32| {
        jump_to(frac);
    };

    let on_landmark_activate = {
        let landmarks = landmarks.clone();
        move |id: ses_shell::LandmarkId| {
            if let Some(lm) = landmarks.iter().find(|lm| lm.id == id) {
                let positions = leaf_positions();
                if let Some(frac) = lm
                    .leaf_ids()
                    .iter()
                    .filter_map(|lid| positions.get(lid).copied())
                    .reduce(f32::min)
                {
                    if lm.focus_on_click {
                        if let Some(leaf_id) = lm.leaf_ids().first().copied() {
                            if let Some(ws) = shell.write().active_mut() {
                                maximize_leaf(ws, leaf_id);
                            }
                        }
                    }
                    jump_to(frac);
                }
            }
        }
    };

    rsx! {
        div {
            class: "ses-page-area",
            tabindex: "0",
            onkeydown: move |evt| {
                if !evt.modifiers().alt() {
                    return;
                }
                let key = evt.key();
                let digit = match key {
                    Key::Character(c) => c.chars().next().and_then(|ch| ch.to_digit(10)),
                    _ => None,
                };
                let Some(n) = digit else {
                    return;
                };
                if !(1..=9).contains(&n) {
                    return;
                }
                let idx = (n - 1) as u8;
                if let Some(lm) = landmarks_for_keys
                    .iter()
                    .find(|lm| lm.shortcut_index == Some(idx))
                {
                    evt.prevent_default();
                    let positions = leaf_positions();
                    if let Some(frac) = lm
                        .leaf_ids()
                        .iter()
                        .filter_map(|id| positions.get(id).copied())
                        .reduce(f32::min)
                    {
                        if lm.focus_on_click {
                            if let Some(leaf_id) = lm.leaf_ids().first().copied() {
                                if let Some(ws) = shell.write().active_mut() {
                                    maximize_leaf(ws, leaf_id);
                                }
                            }
                        }
                        jump_to(frac);
                    }
                }
            },

            if let Some(bar) = top_bar.clone() {
                PageTopBarView {
                    bar,
                    on_action: move |action_id: String| {
                        shell.write().status_message = format!("Top bar: {action_id}");
                    },
                }
            }

            div { class: "ses-page-scroll-host",
                div {
                    class: "ses-page-scroll-content",
                    onmounted: move |e| {
                        scroll_mounted.set(Some(e.data()));
                        measure_leaves();
                    },
                    onscroll: move |evt| {
                        let top = evt.data().scroll_top();
                        let height = evt.data().scroll_height() as f64;
                        let client = evt.data().client_height() as f64;
                        scroll_frac.set(calc_scroll_fraction(top, height, client));
                        let ratio = if height > 0.0 {
                            (client / height).clamp(0.05, 1.0) as f32
                        } else {
                            1.0
                        };
                        thumb_ratio.set(ratio);
                    },
                    PageNodeView { node, path: vec![] }
                }
                LandmarkScrollBar {
                    landmarks: landmarks.clone(),
                    scroll_fraction: scroll_frac,
                    thumb_ratio,
                    on_jump: on_landmark_jump,
                    on_activate: on_landmark_activate,
                    leaf_positions,
                }
            }
        }
    }
}

/// Helper for leaf views: register/unregister mount data.
pub fn register_leaf_mount(mut mounts: LeafMountCtx, id: LeafId, data: Rc<MountedData>) {
    mounts.write().insert(id, data);
}

pub fn unregister_leaf_mount(mut mounts: LeafMountCtx, id: LeafId) {
    mounts.write().remove(&id);
}
