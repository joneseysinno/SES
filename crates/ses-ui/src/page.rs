//! Page split tree rendering.

pub mod leaf;
pub mod node;
pub mod page_area;
pub mod page_view;
pub mod pod_shell;
pub mod pod_stack;
pub mod scroll_bar;
pub mod split_handle;
pub mod top_bar;

pub use leaf::PageLeafView;
pub use node::PageNodeView;
pub use page_area::PageArea;
pub use page_view::PageView;
pub use pod_shell::PodShell;
pub use pod_stack::PodStack;
pub use split_handle::SplitHandle;

use dioxus::prelude::*;
use ses_shell::{PodDescriptor, PodLayout};

/// Default viewport width until page-area measurement exists.
pub const PAGE_PODS_VIEWPORT_PX: u32 = 1200;

/// Render a page's pods with the standard viewport assumption.
pub fn page_pods(
    pods: Vec<PodDescriptor>,
    layout: PodLayout,
    bodies: Vec<(u64, Element)>,
) -> Element {
    rsx! {
        PodStack {
            pods,
            layout,
            viewport_px: PAGE_PODS_VIEWPORT_PX,
            bodies,
        }
    }
}
