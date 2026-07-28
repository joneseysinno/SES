//! SES layout shell — workspaces, page split trees, and data-flow bus.
//!
//! Pure Rust (no Dioxus). UI crates depend on these types for rendering.

pub mod defaults;
pub mod flow;
pub mod ids;
pub mod ops;
pub mod page;
pub mod pod;
pub mod workspace;

pub use defaults::default_shell;
pub use flow::{FlowBus, FlowChannelId, FlowSlot, FlowValue};
pub use ids::{LeafId, ModuleId, WorkspaceId, reset_id_counter};
pub use ops::{
    join_leaf, join_split_at, maximize_leaf, restore_layout, set_leaf_pod, set_split_ratio,
    set_split_ratio_at, split_leaf,
};
pub use page::{Axis, IoLayout, IoPlacement, PageLeaf, PageNode};
pub use pod::{PodDescriptor, PodKind};
pub use workspace::{ShellState, WorkspaceDef};
