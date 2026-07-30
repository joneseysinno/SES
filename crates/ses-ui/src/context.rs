//! Shared Dioxus context aliases.

use crate::module_ui::ModuleUiRegistry;
use dioxus::prelude::*;
use ses_modules::UserContext;
use ses_shell::{FlowBus, ShellState, StartupProfile};
use std::sync::Arc;

pub type ShellCtx = Signal<ShellState>;
pub type FlowCtx = Signal<FlowBus>;
pub type ModulesCtx = Signal<Arc<ModuleUiRegistry>>;
pub type UserCtx = Signal<UserContext>;
pub type StartupCtx = Signal<StartupProfile>;

pub fn use_shell() -> ShellCtx {
    use_context::<ShellCtx>()
}

pub fn use_flow() -> FlowCtx {
    use_context::<FlowCtx>()
}

pub fn use_modules() -> ModulesCtx {
    use_context::<ModulesCtx>()
}

pub fn use_modules_ui() -> ModulesCtx {
    use_modules()
}

pub fn use_user() -> UserCtx {
    use_context::<UserCtx>()
}

pub fn use_startup() -> StartupCtx {
    use_context::<StartupCtx>()
}
