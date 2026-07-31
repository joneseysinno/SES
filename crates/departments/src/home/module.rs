use crate::home::ui::page;
use crate::home::ui::workspace;
use crate::home::MODULE_ID_STR;
use ses_modules::page_manifest::PageManifest;
use ses_modules::permission::Permission;
use ses_modules::SesModule;
use ses_shell::{ModuleId, WorkspaceDef};
use std::sync::OnceLock;

pub struct HomeModule {
    pages: &'static [PageManifest],
}

impl HomeModule {
    pub fn new() -> Self {
        Self {
            pages: home_pages(),
        }
    }
}

impl SesModule for HomeModule {
    fn id(&self) -> ModuleId {
        ModuleId::new(MODULE_ID_STR)
    }

    fn display_name(&self) -> &str {
        "Home"
    }

    fn permission(&self) -> Permission {
        Permission::VIEW | Permission::EDIT
    }

    fn page_manifests(&self) -> &[PageManifest] {
        self.pages
    }

    fn factory_workspaces(&self) -> Vec<WorkspaceDef> {
        workspace::all()
    }

    fn is_template(&self) -> bool {
        false
    }
}

fn home_pages() -> &'static [PageManifest] {
    static PAGES: OnceLock<Vec<PageManifest>> = OnceLock::new();
    PAGES
        .get_or_init(|| {
            vec![
                PageManifest::simple(page::PROJECTS_OVERVIEW, "Projects", Permission::VIEW)
                    .with_description("Portfolio summary across every active project"),
                PageManifest::simple(
                    page::TIMECARD,
                    "Timecard",
                    Permission::VIEW | Permission::EDIT,
                )
                .with_description("Log and review your hours"),
            ]
        })
        .as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roster_matches_manifests() {
        let m = HomeModule::new();
        let manifests = m.page_manifests();
        assert_eq!(manifests.len(), page::ALL.len());

        let mut seen = std::collections::HashSet::new();
        for id in page::ALL {
            assert!(seen.insert(*id));
            assert!(
                manifests.iter().any(|m| m.page_id.as_str() == *id),
                "ALL entry `{id}` missing from manifests"
            );
        }
        for manifest in manifests {
            assert!(
                page::ALL.contains(&manifest.page_id.as_str()),
                "manifest `{}` missing from ALL",
                manifest.page_id.as_str()
            );
            // Both pages must work unbound — that's what makes them
            // dashboard-eligible.
            assert!(manifest.requires.is_empty());
        }
    }
}
