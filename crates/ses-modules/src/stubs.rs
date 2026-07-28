//! Built-in stub modules: core-ui, analysis, documentation.

use crate::permission::Permission;
use crate::registry::SesModule;
use crate::slots::{PodManifest, SlotDecl};
use ses_shell::{ModuleId, PodKind};
use std::sync::OnceLock;

struct StaticModule {
    id: ModuleId,
    name: &'static str,
    permission: Permission,
    manifests: &'static [PodManifest],
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

    fn pod_manifests(&self) -> &[PodManifest] {
        self.manifests
    }
}

fn core_ui_manifests() -> &'static [PodManifest] {
    static M: OnceLock<Vec<PodManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![
            PodManifest::simple(PodKind::View),
            PodManifest::simple(PodKind::Outliner),
            PodManifest::simple(PodKind::Properties),
            PodManifest::simple(PodKind::MenuBar),
            PodManifest::simple(PodKind::TopBar),
            PodManifest::simple(PodKind::StatusBar),
        ]
    })
    .as_slice()
}

fn analysis_manifests() -> &'static [PodManifest] {
    static M: OnceLock<Vec<PodManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![PodManifest::with_io(
            PodKind::Calculation,
            vec![
                SlotDecl::new("a", "Input A"),
                SlotDecl::new("b", "Input B"),
            ],
            vec![SlotDecl::new("result", "Result")],
        )]
    })
    .as_slice()
}

fn documentation_manifests() -> &'static [PodManifest] {
    static M: OnceLock<Vec<PodManifest>> = OnceLock::new();
    M.get_or_init(|| {
        vec![
            PodManifest::simple(PodKind::Outliner),
            PodManifest::simple(PodKind::Properties),
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
            manifests: core_ui_manifests(),
        }),
        Box::new(StaticModule {
            id: ModuleId::new("analysis"),
            name: "Analysis",
            permission: Permission::VIEW.union(Permission::ANALYZE),
            manifests: analysis_manifests(),
        }),
        Box::new(StaticModule {
            id: ModuleId::new("documentation"),
            name: "Documentation",
            permission: Permission::VIEW,
            manifests: documentation_manifests(),
        }),
    ]
}
