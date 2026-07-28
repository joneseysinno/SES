//! Workspace tab strip — switch, rename, context menu, pointer drag-reorder.
//!
//! Uses pointer events (not HTML5 DnD) so reordering works in desktop WebView
//! as well as the browser.

use crate::context::use_shell;
use crate::workspace::switcher::WorkspaceSwitcher;
use dioxus::prelude::*;
use ses_shell::WorkspaceId;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
struct CtxMenu {
    id: WorkspaceId,
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct DragState {
    from: WorkspaceId,
    start_x: f64,
    /// True once the pointer has moved past the drag threshold.
    active: bool,
    /// Insertion index in `[0, len]` before accounting for source removal.
    insert_at: usize,
    /// Client X for the floating insert marker.
    marker_x: f64,
}

const DRAG_THRESHOLD_PX: f64 = 6.0;

/// Returns `(insert_at, marker_x)` for pointer `x`.
///
/// `exclude` is the tab being dragged — it is omitted from slot hit-testing so the
/// marker tracks real gaps when moving left or right past the source.
fn insertion_for_x(
    x: f64,
    edges: &[(usize, WorkspaceId, f64, f64)],
    exclude: Option<WorkspaceId>,
) -> (usize, f64) {
    let edges: Vec<(usize, WorkspaceId, f64, f64)> = edges
        .iter()
        .copied()
        .filter(|(_, id, _, _)| exclude != Some(*id))
        .collect();

    if edges.is_empty() {
        return (0, x);
    }

    // Before the first remaining tab.
    if x < edges[0].2 {
        return (edges[0].0, edges[0].2);
    }

    for i in 0..edges.len() {
        let (orig_idx, _, left, right) = edges[i];
        if x < left {
            // Gap after previous tab / before this one.
            return (orig_idx, left);
        }
        if x < right {
            let mid = (left + right) * 0.5;
            if x < mid {
                return (orig_idx, left);
            }
            return (orig_idx + 1, right);
        }
        // x >= right — fall through toward next tab or end.
    }

    let last = *edges.last().unwrap();
    (last.0 + 1, last.3)
}

fn reorder_to_index(from: usize, insert_at: usize) -> Option<usize> {
    let to = if from >= insert_at {
        insert_at
    } else {
        insert_at.saturating_sub(1)
    };
    if to == from {
        None
    } else {
        Some(to)
    }
}

#[component]
pub fn WorkspaceBar() -> Element {
    let mut shell = use_shell();
    let mut ctx = use_signal(|| None::<CtxMenu>);
    let mut renaming = use_signal(|| None::<WorkspaceId>);
    let mut rename_draft = use_signal(String::new);
    let mut drag = use_signal(|| None::<DragState>);
    let mut tab_rects = use_signal(HashMap::<WorkspaceId, (f64, f64, f64, f64)>::new);
    let mut tab_mounted = use_signal(HashMap::<WorkspaceId, Rc<MountedData>>::new);

    let tabs = {
        let s = shell.read();
        s.workspaces
            .iter()
            .enumerate()
            .map(|(idx, w)| (idx, w.id, w.name.clone(), w.id == s.active_workspace))
            .collect::<Vec<_>>()
    };

    let refresh_rects = move || {
        let mounted = tab_mounted();
        spawn(async move {
            let mut next = HashMap::new();
            for (id, data) in mounted {
                if let Ok(rect) = data.get_client_rect().await {
                    next.insert(
                        id,
                        (
                            rect.min_x(),
                            rect.max_x(),
                            rect.min_y(),
                            rect.height(),
                        ),
                    );
                }
            }
            tab_rects.set(next);
        });
    };

    let compute_insertion = move |x: f64, exclude: Option<WorkspaceId>| -> (usize, f64) {
        let order: Vec<(usize, WorkspaceId)> = shell
            .read()
            .workspaces
            .iter()
            .enumerate()
            .map(|(idx, w)| (idx, w.id))
            .collect();
        let rects = tab_rects();
        let edges: Vec<(usize, WorkspaceId, f64, f64)> = order
            .iter()
            .filter_map(|(idx, id)| {
                rects
                    .get(id)
                    .map(|(left, right, _, _)| (*idx, *id, *left, *right))
            })
            .collect();
        insertion_for_x(x, &edges, exclude)
    };

    let marker_geom = move || -> (f64, f64) {
        // top, height from any measured tab
        tab_rects()
            .values()
            .next()
            .map(|(_, _, top, height)| (*top, *height))
            .unwrap_or((4.0, 28.0))
    };

    let close_ctx = move |_| {
        ctx.set(None);
    };

    rsx! {
        div {
            class: "ses-workspace-bar",
            onpointerdown: move |_| {
                if ctx.read().is_some() {
                    ctx.set(None);
                }
            },
            for (idx, id, name, active) in tabs {
                {
                    let _ = idx;
                    let is_renaming = renaming.read().as_ref() == Some(&id);
                    let is_dragging_source = drag()
                        .map(|d| d.active && d.from == id)
                        .unwrap_or(false);
                    let tab_class = {
                        let mut c = if active {
                            "ses-workspace-tab ses-active".to_string()
                        } else {
                            "ses-workspace-tab".to_string()
                        };
                        if is_dragging_source {
                            c.push_str(" ses-dragging");
                        }
                        c
                    };

                    rsx! {
                        if is_renaming {
                            input {
                                class: "ses-workspace-tab-rename",
                                value: "{rename_draft}",
                                autofocus: true,
                                oninput: move |evt| rename_draft.set(evt.value()),
                                onkeydown: move |evt| {
                                    let key = evt.key();
                                    if key == Key::Enter {
                                        let draft = rename_draft.peek().clone();
                                        if shell.write().rename_workspace(id, draft) {
                                            shell.write().status_message = "Workspace renamed".into();
                                        }
                                        renaming.set(None);
                                    } else if key == Key::Escape {
                                        renaming.set(None);
                                    }
                                },
                                onblur: move |_| {
                                    let draft = rename_draft.peek().clone();
                                    if shell.write().rename_workspace(id, draft) {
                                        shell.write().status_message = "Workspace renamed".into();
                                    }
                                    renaming.set(None);
                                },
                            }
                        } else {
                            button {
                                class: "{tab_class}",
                                onmounted: move |e| {
                                    let data = e.data();
                                    tab_mounted.write().insert(id, data.clone());
                                    spawn(async move {
                                        if let Ok(rect) = data.get_client_rect().await {
                                            tab_rects.write().insert(
                                                id,
                                                (
                                                    rect.min_x(),
                                                    rect.max_x(),
                                                    rect.min_y(),
                                                    rect.height(),
                                                ),
                                            );
                                        }
                                    });
                                },
                                onpointerdown: move |evt| {
                                    if evt.data().trigger_button()
                                        != Some(dioxus::html::input_data::MouseButton::Primary)
                                    {
                                        return;
                                    }
                                    evt.stop_propagation();
                                    let x = evt.client_coordinates().x;
                                    refresh_rects();
                                    let (insert_at, marker_x) = compute_insertion(x, Some(id));
                                    drag.set(Some(DragState {
                                        from: id,
                                        start_x: x,
                                        active: false,
                                        insert_at,
                                        marker_x,
                                    }));
                                },
                                oncontextmenu: move |evt| {
                                    evt.prevent_default();
                                    evt.stop_propagation();
                                    drag.set(None);
                                    let coords = evt.client_coordinates();
                                    ctx.set(Some(CtxMenu {
                                        id,
                                        x: coords.x,
                                        y: coords.y,
                                    }));
                                },
                                "{name}"
                            }
                        }
                    }
                }
            }
            WorkspaceSwitcher {}
        }

        if let Some(state) = drag() {
            if state.active {
                {
                    let (marker_top, marker_height) = marker_geom();
                    rsx! {
                        div {
                            class: "ses-workspace-insert",
                            style: "left: {state.marker_x}px; top: {marker_top}px; height: {marker_height}px;",
                        }
                    }
                }
            }
            div {
                class: "ses-workspace-drag-overlay",
                style: if state.active {
                    "position: fixed; inset: 0; z-index: 150; cursor: grabbing;"
                } else {
                    "position: fixed; inset: 0; z-index: 150; cursor: grab;"
                },
                onpointermove: move |evt| {
                    let x = evt.client_coordinates().x;
                    let Some(mut d) = drag() else {
                        return;
                    };
                    if !d.active && (x - d.start_x).abs() >= DRAG_THRESHOLD_PX {
                        d.active = true;
                    }
                    if d.active {
                        let (insert_at, marker_x) = compute_insertion(x, Some(d.from));
                        d.insert_at = insert_at;
                        d.marker_x = marker_x;
                    }
                    drag.set(Some(d));
                },
                onpointerup: move |evt| {
                    let x = evt.client_coordinates().x;
                    let Some(d) = drag() else {
                        return;
                    };
                    drag.set(None);
                    if d.active {
                        let (insert_at, _) = compute_insertion(x, Some(d.from));
                        let from_idx = shell.read().workspace_index(d.from);
                        if let Some(from) = from_idx {
                            if let Some(to) = reorder_to_index(from, insert_at) {
                                shell.write().reorder_workspace(from, to);
                                shell.write().status_message = "Workspace reordered".into();
                            }
                        }
                    } else {
                        shell.write().set_active(d.from);
                        ctx.set(None);
                    }
                },
                onpointercancel: move |_| {
                    drag.set(None);
                },
            }
        }

        if let Some(menu) = ctx() {
            div {
                class: "ses-workspace-ctx",
                style: "left: {menu.x}px; top: {menu.y}px;",
                onpointerdown: move |evt| evt.stop_propagation(),
                button {
                    onclick: move |_| {
                        let name = shell
                            .read()
                            .workspaces
                            .iter()
                            .find(|w| w.id == menu.id)
                            .map(|w| w.name.clone())
                            .unwrap_or_default();
                        rename_draft.set(name);
                        renaming.set(Some(menu.id));
                        ctx.set(None);
                    },
                    "Rename"
                }
                button {
                    onclick: move |_| {
                        if shell.write().duplicate_workspace(menu.id) {
                            shell.write().status_message = "Workspace duplicated".into();
                        }
                        ctx.set(None);
                    },
                    "Duplicate"
                }
                button {
                    class: "ses-danger-action",
                    onclick: move |_| {
                        if shell.write().remove_workspace(menu.id) {
                            shell.write().status_message = "Workspace removed".into();
                        } else {
                            shell.write().status_message = "Cannot remove last workspace".into();
                        }
                        ctx.set(None);
                    },
                    "Delete"
                }
            }
        }

        if ctx.read().is_some() {
            div {
                style: "position: fixed; inset: 0; z-index: 199;",
                onpointerdown: close_ctx,
            }
        }
    }
}

/// Helper kept for callers that need to switch by id externally.
#[allow(dead_code)]
pub fn switch_workspace(mut shell: Signal<ses_shell::ShellState>, id: WorkspaceId) {
    shell.write().set_active(id);
}

#[cfg(test)]
mod tests {
    use super::{insertion_for_x, reorder_to_index};
    use ses_shell::WorkspaceId;

    #[test]
    fn insertion_uses_midpoint() {
        let a = WorkspaceId(1);
        let b = WorkspaceId(2);
        let edges = vec![(0, a, 0.0, 100.0), (1, b, 100.0, 200.0)];
        assert_eq!(insertion_for_x(10.0, &edges, None).0, 0);
        assert_eq!(insertion_for_x(60.0, &edges, None).0, 1);
        assert_eq!(insertion_for_x(110.0, &edges, None).0, 1);
        assert_eq!(insertion_for_x(160.0, &edges, None).0, 2);
        assert_eq!(insertion_for_x(250.0, &edges, None).0, 2);
    }

    #[test]
    fn insertion_excludes_dragged_tab_when_moving_left() {
        let a = WorkspaceId(1);
        let b = WorkspaceId(2);
        let c = WorkspaceId(3);
        // A[0-100] B[100-200] C[200-300]; dragging C left over B
        let edges = vec![
            (0, a, 0.0, 100.0),
            (1, b, 100.0, 200.0),
            (2, c, 200.0, 300.0),
        ];
        // Over left half of B → insert before B
        assert_eq!(insertion_for_x(120.0, &edges, Some(c)).0, 1);
        assert_eq!(insertion_for_x(120.0, &edges, Some(c)).1, 100.0);
        // Over right half of B → insert after B (back toward C's old slot)
        assert_eq!(insertion_for_x(180.0, &edges, Some(c)).0, 2);
        assert_eq!(insertion_for_x(180.0, &edges, Some(c)).1, 200.0);
        // Over left half of A → insert at start
        assert_eq!(insertion_for_x(20.0, &edges, Some(c)).0, 0);
        // Pointer still over C's old rect is ignored; falls into gap/end handling via B/A only
        assert_eq!(
            reorder_to_index(2, insertion_for_x(120.0, &edges, Some(c)).0),
            Some(1)
        );
    }

    #[test]
    fn insertion_handles_gaps() {
        let a = WorkspaceId(1);
        let b = WorkspaceId(2);
        // Gap between 100 and 120
        let edges = vec![(0, a, 0.0, 100.0), (1, b, 120.0, 220.0)];
        assert_eq!(insertion_for_x(110.0, &edges, None).0, 1);
        assert_eq!(insertion_for_x(110.0, &edges, None).1, 120.0);
    }

    #[test]
    fn reorder_to_accounts_for_removal() {
        // Drag index 0 to end (insert_at=3 for 3 tabs) → to=2
        assert_eq!(reorder_to_index(0, 3), Some(2));
        // Drag index 2 to start (insert_at=0) → to=0
        assert_eq!(reorder_to_index(2, 0), Some(0));
        // Drag index 1 to after itself (insert_at=2) → no-op
        assert_eq!(reorder_to_index(1, 2), None);
        // Drag index 1 to before itself (insert_at=1) → no-op
        assert_eq!(reorder_to_index(1, 1), None);
    }
}
