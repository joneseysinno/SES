//! SES layout shell — workspaces, page split trees, and data-flow bus.
//!
//! Pure Rust (no Dioxus). UI crates depend on these types for rendering.

pub mod defaults;
pub mod flow;
pub mod ids;
pub mod landmark;
pub mod ops;
pub mod page;
pub mod pod;
pub mod workspace;

pub use defaults::default_shell;
pub use flow::{FlowBus, FlowChannelId, FlowSlot, FlowValue};
pub use ids::{LeafId, ModuleId, WorkspaceId, reset_id_counter};
pub use landmark::{LandmarkDef, LandmarkIcon, LandmarkId};
pub use ops::{
    add_landmark, effective_layout, group_landmarks, join_leaf, join_split_at, maximize_leaf,
    remove_landmark, restore_layout, scroll_fraction, set_leaf_collapsed, set_leaf_pod,
    set_split_ratio, set_split_ratio_at, split_leaf,
};
pub use page::{
    Axis, IoLayout, IoPlacement, PageLeaf, PageNode, PageTopBar, TopBarAlign, TopBarHeight,
    TopBarSlot, TopBarSlotKind,
};
pub use pod::{PodDescriptor, PodKind};
pub use workspace::{ShellState, WorkspaceDef};
