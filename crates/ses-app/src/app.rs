//! Root App component — context providers and Screen.

use crate::db::{
    bump_ids_past, load_startup_profile, load_workspaces, save_shell, save_startup_profile,
};
use crate::dept_db::{load_or_seed_dept_store, save_dept_store};
use departments::{DeptStore, DeptStoreCtx};
use dioxus::prelude::*;
use ses_modules::UserContext;
use ses_shell::{
    FlowBus, ShellState, StartupProfile, factory_workspaces, resolve_startup,
};
use ses_ui::{
    FlowCtx, ModuleUiRegistry, ModulesCtx, Screen, ShellCtx, StartupCtx, UserCtx,
};
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
const SCROLL_BAR_CSS: &str = ses_style!("scroll_bar.css");
const PAGE_TOP_BAR_CSS: &str = ses_style!("page_top_bar.css");
const POD_STACK_CSS: &str = ses_style!("pod_stack.css");
const KANBAN_CSS: &str = ses_style!("kanban.css");
const IO_EXTENDED_CSS: &str = ses_style!("io_extended.css");
const MODAL_CSS: &str = ses_style!("modal.css");

#[derive(Clone)]
struct AppInit {
    shell: ShellState,
    modules: Arc<ModuleUiRegistry>,
    profile: StartupProfile,
    dept: DeptStore,
}

fn app_init() -> AppInit {
    #[cfg(feature = "dev-stubs")]
    let mut reg = ModuleUiRegistry::with_defaults();
    #[cfg(not(feature = "dev-stubs"))]
    let mut reg = ModuleUiRegistry::new(ses_modules::ModuleRegistry::new());
    departments::register_all_ui(&mut reg);

    let mut profile = load_startup_profile();
    let mut persisted = load_workspaces();
    if ses_shell::migrate(&mut profile, &mut persisted) {
        save_startup_profile(&profile);
        // Persist the cleaned workspace list via a temporary shell snapshot.
        let cleaned = ShellState {
            workspaces: persisted.clone(),
            active_workspace: persisted
                .first()
                .map(|w| w.id)
                .unwrap_or_else(ses_shell::WorkspaceId::new),
            status_message: "Ready".into(),
            pending_top_bar_actions: Vec::new(),
        };
        save_shell(&cleaned);
    }

    let factory = factory_workspaces(
        &profile.enabled_modules,
        reg.logical.all_factory_workspaces(),
    );
    let shell = resolve_startup(factory, persisted, &profile);
    bump_ids_past(&shell);
    let dept = load_or_seed_dept_store();
    AppInit {
        shell,
        modules: Arc::new(reg),
        profile,
        dept,
    }
}

#[component]
pub fn App() -> Element {
    let init = use_hook(app_init);

    let shell: ShellCtx = use_signal(|| init.shell.clone());
    let flow: FlowCtx = use_signal(FlowBus::new);
    let modules: ModulesCtx = use_signal(|| Arc::clone(&init.modules));
    let user: UserCtx = use_signal(UserContext::dev_all_access);
    let startup: StartupCtx = use_signal(|| init.profile.clone());
    let dept: DeptStoreCtx = use_signal(|| init.dept.clone());

    use_context_provider(|| shell);
    use_context_provider(|| flow);
    use_context_provider(|| modules);
    use_context_provider(|| user);
    use_context_provider(|| startup);
    use_context_provider(|| dept);

    use_effect(move || {
        let snapshot = shell.read().clone();
        save_shell(&snapshot);
    });

    use_effect(move || {
        let profile = startup.read().clone();
        save_startup_profile(&profile);
    });

    use_effect(move || {
        let snapshot = dept.read().clone();
        save_dept_store(&snapshot);
    });

    rsx! {
        style { {SES_CSS} }
        style { {CHROME_CSS} }
        style { {WORKSPACE_CSS} }
        style { {PAGE_CSS} }
        style { {IO_CSS} }
        style { {PODS_CSS} }
        style { {SCROLL_BAR_CSS} }
        style { {PAGE_TOP_BAR_CSS} }
        style { {POD_STACK_CSS} }
        style { {KANBAN_CSS} }
        style { {IO_EXTENDED_CSS} }
        style { {MODAL_CSS} }
        Screen {}
    }
}
