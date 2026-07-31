//! SES business departments.
//!
//! Each department is a module implementing [`ses_modules::SesModule`]. Departments
//! own their pages, payloads, and bridge contracts. Cross-department types live
//! in [`shared`].

pub mod shared;
pub mod store;

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

/// Register every enabled department's logical module only.
pub fn register_all(reg: &mut ModuleRegistry) {
    #[cfg(feature = "project-management")]
    reg.register(Box::new(project_management::ProjectManagementModule::new()));
    #[cfg(feature = "project")]
    reg.register(Box::new(project::ProjectModule::new()));
}

/// Register logical modules and UI renderers together.
pub fn register_all_ui(ui_reg: &mut ModuleUiRegistry) {
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
