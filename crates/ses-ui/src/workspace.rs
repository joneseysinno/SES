//! Workspace tab bar and switcher.

pub mod bar;
pub mod switcher;

pub use bar::WorkspaceBar;
pub use switcher::WorkspaceSwitcher;

use ses_modules::ModuleRegistry;
use ses_shell::{ShellState, StartupProfile};
use crate::module_ui::ModuleUiRegistry;

/// Single reset path used by the workspace switcher and app ribbon.
pub fn reset_to_factory(modules: &ModuleUiRegistry) -> ShellState {
    let factory = ses_shell::factory_workspaces(&[], modules.logical.all_factory_workspaces());
    ses_shell::resolve_startup(factory, Vec::new(), &StartupProfile::default())
}

/// Convenience when only the logical registry is available.
pub fn reset_to_factory_logical(logical: &ModuleRegistry) -> ShellState {
    let factory = ses_shell::factory_workspaces(&[], logical.all_factory_workspaces());
    ses_shell::resolve_startup(factory, Vec::new(), &StartupProfile::default())
}
