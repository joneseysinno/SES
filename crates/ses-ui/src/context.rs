//! Shared Dioxus context aliases.

use dioxus::prelude::*;
use ses_modules::{ModuleRegistry, UserContext};
use ses_shell::{FlowBus, ShellState};
use std::sync::Arc;

pub type ShellCtx = Signal<ShellState>;
pub type FlowCtx = Signal<FlowBus>;
pub type ModulesCtx = Signal<Arc<ModuleRegistry>>;
pub type UserCtx = Signal<UserContext>;

pub fn use_shell() -> ShellCtx {
    use_context::<ShellCtx>()
}

pub fn use_flow() -> FlowCtx {
    use_context::<FlowCtx>()
}

pub fn use_modules() -> ModulesCtx {
    use_context::<ModulesCtx>()
}

pub fn use_user() -> UserCtx {
    use_context::<UserCtx>()
}
