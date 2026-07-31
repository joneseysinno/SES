pub mod bridge;
pub mod module;
pub mod payloads;
pub mod ui;

/// Stable module id string. Persisted in workspace layouts — treat as a data contract.
pub const MODULE_ID_STR: &str = "project-mgmt";

pub use module::ProjectManagementModule;
pub use payloads::{ProjectPhase, ProjectRecord, Proposal, ProposalStatus};
pub use ui::ProjectManagementUi;
