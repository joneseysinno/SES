pub mod bridge;
pub mod module;
pub mod payloads;
pub mod progress;
pub mod ui;

/// Stable module id string. Persisted in workspace layouts — treat as a data contract.
pub const MODULE_ID_STR: &str = "project";

pub use module::ProjectModule;
pub use progress::{compute, ProgressTone, ProjectProgress};
pub use ui::workspace::for_project;
pub use ui::ProjectUi;
