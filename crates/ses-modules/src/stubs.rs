//! Built-in stub modules: core-ui, analysis, documentation.

use crate::page_manifest::PageManifest;
use crate::permission::Permission;
use crate::registry::SesModule;
use crate::slots::{PodManifest, SlotDecl};
use ses_shell::ModuleId;
use std::sync::OnceLock;

struct StaticModule {
    id: ModuleId,
    name: &'static str,
    permission: Permission,
    pages: &'static [PageManifest],
    pods: &'static [PodManifest],
}

impl SesModule for StaticModule {
    fn id(&self) -> ModuleId {
        self.id.clone()
    }

    fn display_name(&self) -> &str {
        self.name
    }

    fn permission(&self) -> Permission {
        self.permission
    }

    fn page_manifests(&self) -> &[PageManifest] {
        self.pages
    }

    fn pod_manifests(&self) -> &[PodManifest] {
        self.pods
    }
}

fn core_ui_pages() -> &'static [PageManifest] {
    static M: OnceLock<Vec<PageManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![
            PageManifest::simple("view", "3D Viewport", Permission::VIEW)
                .with_description("Geometry viewport"),
            PageManifest::simple("outliner", "Outliner", Permission::VIEW)
                .with_description("Scene hierarchy"),
            PageManifest::simple("properties", "Properties", Permission::VIEW)
                .with_description("Object properties"),
            PageManifest::simple("menu-bar", "Menu Bar", Permission::VIEW),
        ]
    })
    .as_slice()
}

fn core_ui_pods() -> &'static [PodManifest] {
    static M: OnceLock<Vec<PodManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![
            PodManifest::simple("top-bar", "Top Bar"),
            PodManifest::simple("status-bar", "Status Bar"),
            PodManifest::simple("menu-bar", "Menu Bar"),
        ]
    })
    .as_slice()
}

fn analysis_pages() -> &'static [PageManifest] {
    static M: OnceLock<Vec<PageManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![
            PageManifest::simple("calculation", "Calculation", Permission::VIEW | Permission::ANALYZE)
                .with_description("Analysis inputs and results"),
            PageManifest::simple("checks", "Checks", Permission::VIEW | Permission::ANALYZE)
                .with_description("Code checks"),
        ]
    })
    .as_slice()
}

fn analysis_pods() -> &'static [PodManifest] {
    static M: OnceLock<Vec<PodManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![PodManifest::with_io(
            "calculation",
            "Calculation",
            vec![
                SlotDecl::new("a", "Input A"),
                SlotDecl::new("b", "Input B"),
            ],
            vec![SlotDecl::new("result", "Result")],
        )]
    })
    .as_slice()
}

fn documentation_pages() -> &'static [PageManifest] {
    static M: OnceLock<Vec<PageManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![
            PageManifest::simple("outliner", "Outliner", Permission::VIEW),
            PageManifest::simple("properties", "Properties", Permission::VIEW),
        ]
    })
    .as_slice()
}

pub fn default_modules() -> Vec<Box<dyn SesModule>> {
    vec![
        Box::new(StaticModule {
            id: ModuleId::new("core-ui"),
            name: "Core UI",
            permission: Permission::VIEW,
            pages: core_ui_pages(),
            pods: core_ui_pods(),
        }),
        Box::new(StaticModule {
            id: ModuleId::new("analysis"),
            name: "Analysis",
            permission: Permission::VIEW.union(Permission::ANALYZE),
            pages: analysis_pages(),
            pods: analysis_pods(),
        }),
        Box::new(StaticModule {
            id: ModuleId::new("documentation"),
            name: "Documentation",
            permission: Permission::VIEW,
            pages: documentation_pages(),
            pods: &[],
        }),
    ]
}
