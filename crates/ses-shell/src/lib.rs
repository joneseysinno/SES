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
pub mod startup;
pub mod workspace;

pub use defaults::{blank_workspace, core_factory_workspaces, default_shell, factory_workspaces};
pub use flow::{FlowBus, FlowChannelId, FlowSlot, FlowValue};
pub use ids::{LeafId, ModuleId, PodId, WorkspaceId, reset_id_counter};
pub use landmark::{LandmarkAnchor, LandmarkDef, LandmarkIcon, LandmarkId};
pub use ops::{
    add_landmark, effective_layout, effective_pod_layout, group_landmarks, join_leaf, join_split_at,
    maximize_leaf, move_pod, ordered_pods, remove_landmark, restore_layout, scroll_fraction,
    set_leaf_page, set_pod_collapsed, set_split_ratio, set_split_ratio_at, split_leaf,
};
pub use page::{
    Axis, IoLayout, IoPlacement, PageDescriptor, PageId, PageLeaf, PageNode, PageTopBar,
    TopBarAlign, TopBarHeight, TopBarSlot, TopBarSlotKind,
};
pub use pod::{PodDescriptor, PodKind, PodLayout};
pub use startup::{StartupProfile, resolve_startup};
pub use workspace::{PageFilter, ShellState, WorkspaceBinding, WorkspaceDef};
