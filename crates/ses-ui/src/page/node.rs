//! Recursive PageNode renderer.

use crate::page::leaf::PageLeafView;
use crate::page::split_handle::SplitHandle;
use dioxus::prelude::*;
use ses_shell::{Axis, PageNode};
use std::rc::Rc;

fn leaf_is_collapsed_strip(node: &PageNode) -> bool {
    matches!(
        node,
        PageNode::Leaf(leaf) if leaf.pod.collapsible && leaf.pod.collapsed
    )
}

fn pane_flex_style(pct: f32, collapsed_strip: bool) -> String {
    if collapsed_strip {
        "flex: 0 0 auto; max-width: 100%; max-height: 100%;".into()
    } else {
        format!("flex: 1 1 {pct}%; max-width: 100%; max-height: 100%;")
    }
}

#[component]
pub fn PageNodeView(node: PageNode, path: Vec<usize>) -> Element {
    match node {
        PageNode::Leaf(leaf) => {
            rsx! {
                PageLeafView { leaf }
            }
        }
        PageNode::Split {
            axis,
            ratio,
            first,
            second,
        } => {
            rsx! {
                SplitNodeView {
                    axis,
                    ratio,
                    first: *first,
                    second: *second,
                    path,
                }
            }
        }
    }
}

#[component]
fn SplitNodeView(
    axis: Axis,
    ratio: f32,
    first: PageNode,
    second: PageNode,
    path: Vec<usize>,
) -> Element {
    let first_pct = (ratio * 100.0).round();
    let second_pct = 100.0 - first_pct;
    let first_collapsed = leaf_is_collapsed_strip(&first);
    let second_collapsed = leaf_is_collapsed_strip(&second);
    let first_style = pane_flex_style(first_pct, first_collapsed);
    let second_style = pane_flex_style(second_pct, second_collapsed);
    let mut path_first = path.clone();
    path_first.push(0);
    let mut path_second = path.clone();
    path_second.push(1);
    let axis_class = axis.css_class();

    let mut parent_extent = use_signal(|| 400.0f64);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);

    let remount = move || {
        if let Some(data) = mounted() {
            spawn(async move {
                if let Ok(rect) = data.get_client_rect().await {
                    let extent = match axis {
                        Axis::Horizontal => rect.width(),
                        Axis::Vertical => rect.height(),
                    };
                    if extent > 1.0 {
                        parent_extent.set(extent);
                    }
                }
            });
        }
    };

    rsx! {
        div {
            class: "ses-page-split {axis_class}",
            onmounted: move |e| {
                let data = e.data();
                mounted.set(Some(data.clone()));
                spawn(async move {
                    if let Ok(rect) = data.get_client_rect().await {
                        let extent = match axis {
                            Axis::Horizontal => rect.width(),
                            Axis::Vertical => rect.height(),
                        };
                        if extent > 1.0 {
                            parent_extent.set(extent);
                        }
                    }
                });
            },
            div {
                class: if first_collapsed {
                    "ses-page-pane ses-collapsed-pane"
                } else {
                    "ses-page-pane"
                },
                style: "{first_style}",
                PageNodeView { node: first, path: path_first }
            }
            SplitHandle {
                axis,
                path: path.clone(),
                ratio,
                parent_extent,
                on_drag_start: move |_| remount(),
            }
            div {
                class: if second_collapsed {
                    "ses-page-pane ses-collapsed-pane"
                } else {
                    "ses-page-pane"
                },
                style: "{second_style}",
                PageNodeView { node: second, path: path_second }
            }
        }
    }
}
