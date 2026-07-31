//! SES business departments.
//!
//! Each department is a module implementing [`ses_modules::SesModule`]. Departments
//! own their pages, payloads, and bridge contracts. Cross-department types live
//! in [`shared`].

pub mod shared;
pub mod store;

#[cfg(feature = "home")]
pub mod home;
#[cfg(feature = "project-management")]
pub mod project_management;
#[cfg(feature = "project")]
pub mod project;

pub use store::{
    use_dept_store, DeptStore, DeptStoreCtx, MgmtQueryResult, ProjectQueryResult, StoreEffect,
    StoreError,
};

use ses_modules::ModuleRegistry;
use ses_ui::ModuleUiRegistry;

/// Deterministic factory tab order across every department. Lower sorts
/// first. Add a new department's constant here rather than hardcoding a
/// number in its `workspace.rs`.
pub mod seed_order {
    pub const HOME: i16 = 10;
    pub const PROJECT_MANAGEMENT: i16 = 20;
}

/// Register every enabled department's logical module only. `home` goes
/// first — it has no factory workspaces of its own to seed ahead of it, but
/// registration order is also page-picker order.
pub fn register_all(reg: &mut ModuleRegistry) {
    #[cfg(feature = "home")]
    reg.register(Box::new(home::HomeModule::new()));
    #[cfg(feature = "project-management")]
    reg.register(Box::new(project_management::ProjectManagementModule::new()));
    #[cfg(feature = "project")]
    reg.register(Box::new(project::ProjectModule::new()));
}

/// Register logical modules and UI renderers together.
pub fn register_all_ui(ui_reg: &mut ModuleUiRegistry) {
    #[cfg(feature = "home")]
    {
        ui_reg.register_logical(Box::new(home::HomeModule::new()));
        ui_reg.register_ui(Box::new(home::HomeUi));
    }
    #[cfg(feature = "project-management")]
    {
        ui_reg.register_logical(Box::new(project_management::ProjectManagementModule::new()));
        ui_reg.register_ui(Box::new(project_management::ProjectManagementUi));
    }
    #[cfg(feature = "project")]
    {
        ui_reg.register_logical(Box::new(project::ProjectModule::new()));
        ui_reg.register_ui(Box::new(project::ProjectUi));
    }
}
