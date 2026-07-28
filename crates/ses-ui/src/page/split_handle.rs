//! Drag-to-resize split divider + Blender-style directional join.

use crate::context::use_shell;
use dioxus::prelude::*;
use ses_shell::{
    Axis, PageNode,
    ops::{join_split_at, restore_layout, set_split_ratio_at},
};

const JOIN_DEAD_ZONE_PX: f64 = 8.0;

#[derive(Clone, Copy)]
struct JoinMode {
    center_x: f64,
    center_y: f64,
    /// Parent split bounds for arrow placement.
    parent_left: f64,
    parent_top: f64,
    parent_width: f64,
    parent_height: f64,
    /// None until mouse leaves the dead zone.
    discard_first: Option<bool>,
    /// Ignore the pointerup from the right-click that opened join mode.
    ignore_secondary_up: bool,
}

#[component]
pub fn SplitHandle(
    axis: Axis,
    path: Vec<usize>,
    ratio: f32,
    parent_extent: Signal<f64>,
    on_drag_start: EventHandler<()>,
) -> Element {
    let mut shell = use_shell();
    let axis_class = axis.css_class();
    let mut dragging = use_signal(|| false);
    let mut joining = use_signal(|| None::<JoinMode>);
    let mut start_pos = use_signal(|| 0.0f64);
    let mut start_ratio = use_signal(|| ratio);
    let path = use_signal(|| path);
    let mut handle_mounted = use_signal(|| None::<std::rc::Rc<MountedData>>);

    let cursor = match axis {
        Axis::Horizontal => "col-resize",
        Axis::Vertical => "row-resize",
    };

    let mut apply_at = move |pos: f64| {
        let extent = parent_extent().max(1.0);
        let delta = pos - start_pos();
        let next = (start_ratio() as f64 + delta / extent).clamp(0.05, 0.95) as f32;
        let path = path();
        let mut s = shell.write();
        if let Some(ws) = s.active_mut() {
            set_split_ratio_at(&mut ws.layout, &path, next);
        }
    };

    let mut confirm_join = move |discard_first: bool| {
        let path = path();
        let mut s = shell.write();
        if let Some(ws) = s.active_mut() {
            if ws.maximized.is_some() {
                restore_layout(ws);
            }
            if join_split_at(&mut ws.layout, &path, discard_first) {
                s.status_message = if discard_first {
                    "Joined — removed first pane".into()
                } else {
                    "Joined — removed second pane".into()
                };
            } else {
                s.status_message = "Join failed".into();
            }
        }
        joining.set(None);
    };

    let mut start_join_mode = move |cx: f64, cy: f64| {
        let extent = parent_extent().max(40.0);
        let ratio_now = ratio as f64;
        let (parent_left, parent_top, parent_width, parent_height) = match axis {
            Axis::Horizontal => (cx - extent * ratio_now, cy - 20.0, extent, 40.0),
            Axis::Vertical => (cx - 20.0, cy - extent * ratio_now, 40.0, extent),
        };
        joining.set(Some(JoinMode {
            center_x: cx,
            center_y: cy,
            parent_left,
            parent_top,
            parent_width,
            parent_height,
            discard_first: None,
            ignore_secondary_up: true,
        }));

        // Refine geometry from the handle element once available.
        let handle = handle_mounted();
        let ratio_now = ratio;
        let extent = parent_extent();
        spawn(async move {
            let Some(data) = handle else {
                return;
            };
            let Ok(rect) = data.get_client_rect().await else {
                return;
            };
            let cx = (rect.min_x() + rect.max_x()) * 0.5;
            let cy = (rect.min_y() + rect.max_y()) * 0.5;
            let (parent_left, parent_top, parent_width, parent_height) = match axis {
                Axis::Horizontal => {
                    let w = extent.max(rect.width());
                    let first_w = w * ratio_now as f64;
                    (
                        rect.min_x() - first_w,
                        rect.min_y(),
                        w,
                        rect.height().max(40.0),
                    )
                }
                Axis::Vertical => {
                    let h = extent.max(rect.height());
                    let first_h = h * ratio_now as f64;
                    (
                        rect.min_x(),
                        rect.min_y() - first_h,
                        rect.width().max(40.0),
                        h,
                    )
                }
            };
            if let Some(mut m) = joining() {
                m.center_x = cx;
                m.center_y = cy;
                m.parent_left = parent_left;
                m.parent_top = parent_top;
                m.parent_width = parent_width;
                m.parent_height = parent_height;
                joining.set(Some(m));
            }
        });
    };

    rsx! {
        div {
            class: if dragging() {
                "ses-split-handle {axis_class} ses-dragging"
            } else if joining().is_some() {
                "ses-split-handle {axis_class} ses-joining"
            } else {
                "ses-split-handle {axis_class}"
            },
            title: "Drag to resize · Right-click to join, then left-click to confirm",
            onmounted: move |e| {
                handle_mounted.set(Some(e.data()));
            },
            onpointerdown: move |evt| {
                evt.stop_propagation();
                let btn = evt.data().trigger_button();
                if btn == Some(dioxus::html::input_data::MouseButton::Secondary) {
                    evt.prevent_default();
                    let coords = evt.client_coordinates();
                    start_join_mode(coords.x, coords.y);
                    return;
                }
                if btn != Some(dioxus::html::input_data::MouseButton::Primary) {
                    return;
                }
                // If join mode is open, let the overlay handle the click.
                if joining().is_some() {
                    return;
                }
                on_drag_start.call(());
                let coords = evt.client_coordinates();
                let pos = match axis {
                    Axis::Horizontal => coords.x,
                    Axis::Vertical => coords.y,
                };
                start_pos.set(pos);
                start_ratio.set(ratio);
                dragging.set(true);
            },
            oncontextmenu: move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
            },
        }

        if dragging() {
            div {
                style: "position: fixed; inset: 0; z-index: 1000; cursor: {cursor};",
                onpointermove: move |evt| {
                    evt.stop_propagation();
                    let coords = evt.client_coordinates();
                    let pos = match axis {
                        Axis::Horizontal => coords.x,
                        Axis::Vertical => coords.y,
                    };
                    apply_at(pos);
                },
                onpointerup: move |evt| {
                    evt.stop_propagation();
                    let coords = evt.client_coordinates();
                    let pos = match axis {
                        Axis::Horizontal => coords.x,
                        Axis::Vertical => coords.y,
                    };
                    apply_at(pos);
                    dragging.set(false);
                    let path = path();
                    let mut s = shell.write();
                    if let Some(ws) = s.active() {
                        if let Some(r) = ratio_at(&ws.layout, &path) {
                            s.status_message = format!("Split ratio → {:.0}%", r * 100.0);
                        }
                    }
                },
                onpointercancel: move |_| {
                    dragging.set(false);
                },
            }
        }

        if let Some(mode) = joining() {
            {
                let arrow_dir = match (axis, mode.discard_first) {
                    (Axis::Horizontal, Some(true)) => Some("left"),
                    (Axis::Horizontal, Some(false)) => Some("right"),
                    (Axis::Vertical, Some(true)) => Some("up"),
                    (Axis::Vertical, Some(false)) => Some("down"),
                    (_, None) => None,
                };

                let (arrow_left, arrow_top, arrow_w, arrow_h) = match (axis, mode.discard_first) {
                    (Axis::Horizontal, Some(true)) => (
                        mode.parent_left,
                        mode.parent_top,
                        mode.center_x - mode.parent_left,
                        mode.parent_height,
                    ),
                    (Axis::Horizontal, Some(false)) => (
                        mode.center_x,
                        mode.parent_top,
                        mode.parent_left + mode.parent_width - mode.center_x,
                        mode.parent_height,
                    ),
                    (Axis::Vertical, Some(true)) => (
                        mode.parent_left,
                        mode.parent_top,
                        mode.parent_width,
                        mode.center_y - mode.parent_top,
                    ),
                    (Axis::Vertical, Some(false)) => (
                        mode.parent_left,
                        mode.center_y,
                        mode.parent_width,
                        mode.parent_top + mode.parent_height - mode.center_y,
                    ),
                    (_, None) => (0.0, 0.0, 0.0, 0.0),
                };

                rsx! {
                    div {
                        class: "ses-join-overlay",
                        tabindex: "0",
                        autofocus: true,
                        oncontextmenu: move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                        },
                        onpointermove: move |evt| {
                            evt.prevent_default();
                            let coords = evt.client_coordinates();
                            let Some(mut m) = joining() else {
                                return;
                            };
                            let (delta, toward_first) = match axis {
                                Axis::Horizontal => {
                                    let d = coords.x - m.center_x;
                                    (d.abs(), d < 0.0)
                                }
                                Axis::Vertical => {
                                    let d = coords.y - m.center_y;
                                    (d.abs(), d < 0.0)
                                }
                            };
                            if delta >= JOIN_DEAD_ZONE_PX {
                                m.discard_first = Some(toward_first);
                            }
                            joining.set(Some(m));
                        },
                        onpointerup: move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            let btn = evt.data().trigger_button();
                            if btn == Some(dioxus::html::input_data::MouseButton::Secondary) {
                                // Swallow the release of the opening right-click.
                                if let Some(mut m) = joining() {
                                    if m.ignore_secondary_up {
                                        m.ignore_secondary_up = false;
                                        joining.set(Some(m));
                                        return;
                                    }
                                }
                                // A later right-click cancels join mode.
                                joining.set(None);
                                return;
                            }
                            if btn != Some(dioxus::html::input_data::MouseButton::Primary) {
                                return;
                            }
                            if let Some(m) = joining() {
                                if let Some(discard_first) = m.discard_first {
                                    confirm_join(discard_first);
                                    return;
                                }
                            }
                            // Left-click in dead zone — cancel.
                            joining.set(None);
                        },
                        onkeydown: move |evt| {
                            if evt.key() == Key::Escape {
                                joining.set(None);
                            }
                        },
                        if let Some(dir) = arrow_dir {
                            div {
                                class: "ses-join-target ses-join-{dir}",
                                style: "left: {arrow_left}px; top: {arrow_top}px; width: {arrow_w}px; height: {arrow_h}px;",
                                div { class: "ses-join-arrow ses-join-arrow-{dir}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn ratio_at(node: &PageNode, path: &[usize]) -> Option<f32> {
    let mut cur = node;
    for &idx in path {
        match cur {
            PageNode::Split { first, second, .. } => {
                cur = if idx == 0 {
                    first.as_ref()
                } else {
                    second.as_ref()
                };
            }
            PageNode::Leaf(_) => return None,
        }
    }
    match cur {
        PageNode::Split { ratio, .. } => Some(*ratio),
        PageNode::Leaf(_) => None,
    }
}
