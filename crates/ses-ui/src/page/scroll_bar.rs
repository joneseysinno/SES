//! Custom landmark scroll bar for the page area.

use dioxus::prelude::*;
use ses_shell::{LandmarkDef, LandmarkId, LeafId};
use std::collections::HashMap;

fn landmark_span(lm: &LandmarkDef, positions: &HashMap<LeafId, f32>) -> Option<(f32, f32)> {
    let fracs: Vec<f32> = lm
        .leaf_ids
        .iter()
        .filter_map(|id| positions.get(id).copied())
        .collect();
    if fracs.is_empty() {
        return None;
    }
    let min_f = fracs.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_f = fracs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    Some((min_f, max_f))
}

#[component]
pub fn LandmarkScrollBar(
    landmarks: Vec<LandmarkDef>,
    scroll_fraction: Signal<f32>,
    thumb_ratio: Signal<f32>,
    on_jump: EventHandler<f32>,
    on_activate: EventHandler<LandmarkId>,
    leaf_positions: Signal<HashMap<LeafId, f32>>,
) -> Element {
    let mut dragging = use_signal(|| false);
    let mut bar_mounted = use_signal(|| None::<std::rc::Rc<MountedData>>);

    let thumb_h = {
        let ratio = thumb_ratio().clamp(0.05, 1.0);
        (ratio * 100.0).clamp(8.0, 100.0)
    };
    let thumb_top = {
        let frac = scroll_fraction().clamp(0.0, 1.0);
        let travel = (100.0 - thumb_h).max(0.0);
        frac * travel
    };

    let positions = leaf_positions();

    let brackets: Vec<(f32, f32, String)> = landmarks
        .iter()
        .filter(|lm| lm.leaf_ids.len() > 1)
        .filter_map(|lm| {
            let (min_f, max_f) = landmark_span(lm, &positions)?;
            let color = lm
                .icon
                .color
                .clone()
                .unwrap_or_else(|| "var(--ses-accent)".into());
            Some((min_f, max_f, color))
        })
        .collect();

    let icons: Vec<(LandmarkId, f32, String, String, Option<String>)> = landmarks
        .iter()
        .filter_map(|lm| {
            let (min_f, _) = landmark_span(lm, &positions)?;
            let label = lm.icon.label.clone();
            let tip = lm.tooltip.clone().unwrap_or_else(|| label.clone());
            Some((lm.id, min_f, label, tip, lm.icon.color.clone()))
        })
        .collect();

    let jump_from_y = move |client_y: f64| {
        let Some(data) = bar_mounted() else {
            return;
        };
        spawn(async move {
            if let Ok(rect) = data.get_client_rect().await {
                let h = rect.height().max(1.0);
                let y = (client_y - rect.min_y()).clamp(0.0, h);
                let frac = (y / h) as f32;
                on_jump.call(frac.clamp(0.0, 1.0));
            }
        });
    };

    rsx! {
        div {
            class: "ses-landmark-bar",
            onmounted: move |e| {
                bar_mounted.set(Some(e.data()));
            },
            onpointerdown: move |evt| {
                if evt.data().trigger_button()
                    != Some(dioxus::html::input_data::MouseButton::Primary)
                {
                    return;
                }
                jump_from_y(evt.client_coordinates().y);
            },

            div { class: "ses-landmark-track" }

            for (min_f, max_f, color) in brackets {
                {
                    let top = (min_f * 100.0).clamp(0.0, 100.0);
                    let height = ((max_f - min_f) * 100.0).clamp(2.0, 100.0 - top);
                    rsx! {
                        div {
                            class: "ses-landmark-bracket",
                            style: "top: {top}%; height: {height}%; background: {color};",
                        }
                    }
                }
            }

            for (id, min_f, label, tip, color) in icons {
                {
                    let top = (min_f * 100.0).clamp(0.0, 100.0);
                    let color_style = color
                        .as_ref()
                        .map(|c| format!("border-color: {c};"))
                        .unwrap_or_default();
                    rsx! {
                        button {
                            class: "ses-landmark-icon",
                            style: "top: {top}%; {color_style}",
                            title: "{tip}",
                            onpointerdown: move |evt| evt.stop_propagation(),
                            onclick: move |evt| {
                                evt.stop_propagation();
                                on_activate.call(id);
                            },
                            "{label}"
                        }
                    }
                }
            }

            div {
                class: if dragging() {
                    "ses-landmark-thumb ses-dragging"
                } else {
                    "ses-landmark-thumb"
                },
                style: "top: {thumb_top}%; height: {thumb_h}%;",
                onpointerdown: move |evt| {
                    evt.stop_propagation();
                    if evt.data().trigger_button()
                        != Some(dioxus::html::input_data::MouseButton::Primary)
                    {
                        return;
                    }
                    dragging.set(true);
                    jump_from_y(evt.client_coordinates().y);
                },
            }

            if dragging() {
                div {
                    style: "position: fixed; inset: 0; z-index: 200; cursor: grabbing;",
                    onpointermove: move |evt| {
                        jump_from_y(evt.client_coordinates().y);
                    },
                    onpointerup: move |_| dragging.set(false),
                    onpointercancel: move |_| dragging.set(false),
                }
            }
        }
    }
}
