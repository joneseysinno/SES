//! Module trait and registry.

use crate::permission::Permission;
use crate::slots::PodManifest;
use crate::user::UserContext;
use ses_shell::ModuleId;

pub trait SesModule: Send + Sync {
    fn id(&self) -> ModuleId;
    fn display_name(&self) -> &str;
    fn permission(&self) -> Permission;
    fn pod_manifests(&self) -> &[PodManifest];
}

pub struct ModuleRegistry {
    modules: Vec<Box<dyn SesModule>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut reg = Self::new();
        for m in crate::stubs::default_modules() {
            reg.register(m);
        }
        reg
    }

    pub fn register(&mut self, module: Box<dyn SesModule>) {
        self.modules.push(module);
    }

    pub fn modules(&self) -> &[Box<dyn SesModule>] {
        &self.modules
    }

    pub fn get(&self, id: &ModuleId) -> Option<&dyn SesModule> {
        self.modules
            .iter()
            .find(|m| m.id() == *id)
            .map(|m| m.as_ref())
    }

    pub fn visible_modules(&self, user: &UserContext) -> Vec<&dyn SesModule> {
        self.modules
            .iter()
            .filter(|m| user.can_use(m.permission()))
            .map(|m| m.as_ref())
            .collect()
    }

    pub fn module_id_for_pod(&self, kind: ses_shell::PodKind) -> Option<ModuleId> {
        for m in &self.modules {
            if m.pod_manifests().iter().any(|p| p.kind == kind) {
                return Some(m.id());
            }
        }
        None
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
