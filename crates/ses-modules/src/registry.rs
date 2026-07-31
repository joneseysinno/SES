//! Module trait and registry.

use crate::page_manifest::PageManifest;
use crate::permission::Permission;
use crate::slots::PodManifest;
use crate::user::UserContext;
use ses_shell::{ModuleId, PageFilter, WorkspaceDef};

pub trait SesModule: Send + Sync {
    fn id(&self) -> ModuleId;
    fn display_name(&self) -> &str;
    fn permission(&self) -> Permission;

    /// Pages this department offers to the page picker.
    fn page_manifests(&self) -> &[PageManifest] {
        &[]
    }

    /// Workspaces seeded on first enable. Factory layer only —
    /// user edits override these via StartupProfile.
    fn factory_workspaces(&self) -> Vec<WorkspaceDef> {
        Vec::new()
    }

    /// The unbound template this module instantiates per record.
    /// `Some` iff `is_template()`. Never returned from `factory_workspaces()`.
    fn template_workspace(&self) -> Option<WorkspaceDef> {
        None
    }

    /// Every top-bar `action_id` this module's pages know how to handle.
    /// Audited against the module's factory workspaces — see tests.
    fn top_bar_actions(&self) -> &[&'static str] {
        &[]
    }

    /// Is this department a per-instance template? If true, the shell offers
    /// "Open as workspace" from a record, cloning the template and binding it.
    fn is_template(&self) -> bool {
        false
    }

    /// Retained for shell chrome / legacy I/O slot declarations.
    fn pod_manifests(&self) -> &[PodManifest] {
        &[]
    }
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

    /// Pages offered by the workspace's page filter, gated by permission and binding.
    pub fn pages_for_workspace(
        &self,
        ws: &WorkspaceDef,
        user: &UserContext,
    ) -> Vec<(ModuleId, &PageManifest)> {
        let candidates: Vec<&dyn SesModule> = match &ws.page_filter {
            PageFilter::Department(m) => self.get(m).into_iter().collect(),
            PageFilter::UserDefined => self.visible_modules(user),
        };
        candidates
            .iter()
            .flat_map(|m| m.page_manifests().iter().map(move |p| (m.id(), p)))
            .filter(|(_, p)| user.can_use(p.permission))
            .filter(|(_, p)| p.requires.iter().all(|k| ws.binding.get(k).is_some()))
            .collect()
    }

    /// Collect factory workspaces from all registered modules.
    pub fn all_factory_workspaces(&self) -> Vec<WorkspaceDef> {
        self.modules
            .iter()
            .flat_map(|m| m.factory_workspaces())
            .collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permission::Permission;
    use ses_shell::{PageDescriptor, PageNode, TopBarSlotKind, WorkspaceBinding};

    struct FakeMod {
        id: ModuleId,
        pages: Vec<PageManifest>,
    }

    impl SesModule for FakeMod {
        fn id(&self) -> ModuleId {
            self.id.clone()
        }
        fn display_name(&self) -> &str {
            "Fake"
        }
        fn permission(&self) -> Permission {
            Permission::VIEW
        }
        fn page_manifests(&self) -> &[PageManifest] {
            &self.pages
        }
    }

    #[test]
    fn pages_for_workspace_respects_department_filter_and_binding() {
        let mut reg = ModuleRegistry::new();
        reg.register(Box::new(FakeMod {
            id: ModuleId::new("project"),
            pages: vec![
                PageManifest::simple("overview", "Overview", Permission::VIEW),
                PageManifest::simple("task-board", "Board", Permission::VIEW)
                    .with_requires(&["project_id"]),
            ],
        }));
        reg.register(Box::new(FakeMod {
            id: ModuleId::new("other"),
            pages: vec![PageManifest::simple("x", "X", Permission::VIEW)],
        }));

        let mut ws = WorkspaceDef::new(
            "P",
            PageNode::leaf(PageDescriptor::new("project", "overview")),
        )
        .for_department("project");

        let user = UserContext::dev_all_access();
        let pages = reg.pages_for_workspace(&ws, &user);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].1.page_id.as_str(), "overview");

        ws.binding
            .set(WorkspaceBinding::PROJECT_ID, "42");
        let pages = reg.pages_for_workspace(&ws, &user);
        assert_eq!(pages.len(), 2);
    }

    #[test]
    fn template_invariant_holds_for_registered_modules() {
        let reg = ModuleRegistry::with_defaults();
        for m in reg.modules() {
            assert_eq!(
                m.is_template(),
                m.template_workspace().is_some(),
                "module {} template invariant",
                m.id()
            );
            if m.is_template() {
                assert!(
                    m.factory_workspaces().is_empty(),
                    "template module {} must seed no factory workspaces",
                    m.id()
                );
            }
        }
    }

    #[test]
    fn top_bar_actions_cover_factory_buttons() {
        let reg = ModuleRegistry::with_defaults();
        for m in reg.modules() {
            let declared = m.top_bar_actions();
            for ws in m.factory_workspaces() {
                let Some(bar) = ws.top_bar else { continue };
                for slot in bar.slots {
                    if let TopBarSlotKind::Button { action_id, .. } = slot.kind {
                        assert!(
                            declared.contains(&action_id.as_str()),
                            "module {} button `{action_id}` missing from top_bar_actions()",
                            m.id()
                        );
                    }
                }
            }
        }
    }
}
