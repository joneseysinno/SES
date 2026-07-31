//! Home reads. Home does not own.
//!
//! This module declares pages and nothing else. It has no `payloads/` and no
//! `bridge/`. Every number it shows is queried from another department's
//! store; every write it performs is dispatched as another department's
//! command. If a piece of state has no home outside this module, it does not
//! belong in a dashboard — it belongs in the department that owns the concept.

pub mod module;
pub mod ui;

/// Stable module id string. Persisted in workspace layouts — treat as a data contract.
pub const MODULE_ID_STR: &str = "home";

pub use module::HomeModule;
pub use ui::HomeUi;
