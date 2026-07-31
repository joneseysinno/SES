//! Module UI trait — keeps ses-modules Dioxus-free.

use dioxus::prelude::*;
use ses_modules::{ModuleRegistry, SesModule, UserContext};
use ses_shell::{LeafId, ModuleId, PageId, PodLayout, WorkspaceBinding, WorkspaceId};
use std::collections::HashMap;

/// Context passed to [`SesModuleUi::render_page`].
#[derive(Clone, PartialEq)]
pub struct PageCtx {
    pub workspace_id: WorkspaceId,
    pub leaf_id: LeafId,
    pub binding: WorkspaceBinding,
    pub user: UserContext,
    pub pod_layout: PodLayout,
}

impl PageCtx {
    pub fn binding_get(&self, key: &str) -> Option<&str> {
        self.binding.get(key)
    }
}

/// UI half of a department module. Registered alongside [`SesModule`].
pub trait SesModuleUi: Send + Sync {
    fn module_id(&self) -> ModuleId;

    /// Render a page. The shell calls this when a leaf holds a
    /// PageDescriptor whose module_id matches.
    fn render_page(&self, page_id: &PageId, ctx: &PageCtx) -> Element;
}

/// Holds both logical modules and their UI renderers.
pub struct ModuleUiRegistry {
    pub logical: ModuleRegistry,
    ui: HashMap<String, Box<dyn SesModuleUi>>,
}

impl ModuleUiRegistry {
    pub fn new(logical: ModuleRegistry) -> Self {
        Self {
            logical,
            ui: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut reg = Self::new(ModuleRegistry::with_defaults());
        reg.register_ui(Box::new(stub_ui::CoreUiPages));
        reg.register_ui(Box::new(stub_ui::AnalysisPages));
        reg.register_ui(Box::new(stub_ui::DocumentationPages));
        reg
    }

    pub fn register_logical(&mut self, module: Box<dyn SesModule>) {
        self.logical.register(module);
    }

    pub fn register_ui(&mut self, ui: Box<dyn SesModuleUi>) {
        self.ui.insert(ui.module_id().0.clone(), ui);
    }

    pub fn render_page(&self, module_id: &ModuleId, page_id: &PageId, ctx: &PageCtx) -> Element {
        if let Some(ui) = self.ui.get(module_id.as_str()) {
            return ui.render_page(page_id, ctx);
        }
        rsx! {
            div { class: "ses-pod",
                p { class: "ses-muted",
                    "No UI registered for module '{module_id}' page '{page_id}'."
                }
            }
        }
    }
}

pub mod stub_ui {
    use super::*;
    use crate::pod::{
        calculation::CalculationPod, menu_bar::MenuBarPod, outliner::OutlinerPod,
        properties::PropertiesPod, view::ViewPod,
    };

    pub struct CoreUiPages;
    impl SesModuleUi for CoreUiPages {
        fn module_id(&self) -> ModuleId {
            ModuleId::new("core-ui")
        }
        fn render_page(&self, page_id: &PageId, _ctx: &PageCtx) -> Element {
            match page_id.as_str() {
                "view" => rsx! { ViewPod {} },
                "outliner" => rsx! { OutlinerPod {} },
                "properties" => rsx! { PropertiesPod {} },
                "menu-bar" => rsx! { MenuBarPod {} },
                other => rsx! {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "Unknown core-ui page: {other}" }
                    }
                },
            }
        }
    }

    pub struct AnalysisPages;
    impl SesModuleUi for AnalysisPages {
        fn module_id(&self) -> ModuleId {
            ModuleId::new("analysis")
        }
        fn render_page(&self, page_id: &PageId, _ctx: &PageCtx) -> Element {
            let channel = "calc.result".to_string();
            match page_id.as_str() {
                "calculation" | "checks" => rsx! { CalculationPod { channel } },
                other => rsx! {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "Unknown analysis page: {other}" }
                    }
                },
            }
        }
    }

    pub struct DocumentationPages;
    impl SesModuleUi for DocumentationPages {
        fn module_id(&self) -> ModuleId {
            ModuleId::new("documentation")
        }
        fn render_page(&self, page_id: &PageId, _ctx: &PageCtx) -> Element {
            match page_id.as_str() {
                "outliner" => rsx! { OutlinerPod {} },
                "properties" => rsx! { PropertiesPod {} },
                other => rsx! {
                    div { class: "ses-pod",
                        p { class: "ses-muted", "Unknown documentation page: {other}" }
                    }
                },
            }
        }
    }
}
