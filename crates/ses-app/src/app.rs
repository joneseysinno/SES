//! Root App component — context providers and Screen.

use crate::db::{load_or_default_shell, save_shell};
use dioxus::prelude::*;
use ses_modules::{ModuleRegistry, UserContext};
use ses_shell::FlowBus;
use ses_ui::{FlowCtx, ModulesCtx, Screen, ShellCtx, UserCtx};
use std::sync::Arc;

macro_rules! ses_style {
    ($file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/styles/",
            $file
        ))
    };
}

const SES_CSS: &str = ses_style!("ses.css");
const CHROME_CSS: &str = ses_style!("chrome.css");
const WORKSPACE_CSS: &str = ses_style!("workspace.css");
const PAGE_CSS: &str = ses_style!("page.css");
const IO_CSS: &str = ses_style!("io.css");
const PODS_CSS: &str = ses_style!("pods.css");

#[component]
pub fn App() -> Element {
    let shell: ShellCtx = use_signal(load_or_default_shell);
    let flow: FlowCtx = use_signal(FlowBus::new);
    let modules: ModulesCtx = use_signal(|| Arc::new(ModuleRegistry::with_defaults()));
    let user: UserCtx = use_signal(UserContext::dev_all_access);

    use_context_provider(|| shell);
    use_context_provider(|| flow);
    use_context_provider(|| modules);
    use_context_provider(|| user);

    use_effect(move || {
        let snapshot = shell.read().clone();
        save_shell(&snapshot);
    });

    rsx! {
        style { {SES_CSS} }
        style { {CHROME_CSS} }
        style { {WORKSPACE_CSS} }
        style { {PAGE_CSS} }
        style { {IO_CSS} }
        style { {PODS_CSS} }
        Screen {}
    }
}
