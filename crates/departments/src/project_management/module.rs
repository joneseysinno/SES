use crate::project_management::ui::page;
use crate::project_management::ui::workspace;
use crate::project_management::MODULE_ID_STR;
use ses_modules::page_manifest::PageManifest;
use ses_modules::permission::Permission;
use ses_modules::SesModule;
use ses_shell::{ModuleId, WorkspaceDef};
use std::sync::OnceLock;

pub struct ProjectManagementModule {
    pages: &'static [PageManifest],
}

impl ProjectManagementModule {
    pub fn new() -> Self {
        Self {
            pages: mgmt_pages(),
        }
    }
}

impl SesModule for ProjectManagementModule {
    fn id(&self) -> ModuleId {
        ModuleId::new(MODULE_ID_STR)
    }

    fn display_name(&self) -> &str {
        "Project Management"
    }

    fn permission(&self) -> Permission {
        Permission::VIEW | Permission::EDIT | Permission::ADMIN
    }

    fn page_manifests(&self) -> &[PageManifest] {
        self.pages
    }

    fn factory_workspaces(&self) -> Vec<WorkspaceDef> {
        workspace::all()
    }

    fn top_bar_actions(&self) -> &[&'static str] {
        &["new-project", "new-proposal"]
    }

    fn is_template(&self) -> bool {
        false
    }
}

fn mgmt_pages() -> &'static [PageManifest] {
    static PAGES: OnceLock<Vec<PageManifest>> = OnceLock::new();
    PAGES
        .get_or_init(|| {
            vec![
                PageManifest::simple(
                    page::PORTFOLIO_OVERVIEW,
                    "Portfolio",
                    Permission::VIEW,
                )
                .with_description("Firm-wide progress summary and active project list"),
                PageManifest::simple(page::PROJECT_BOARD, "Project Board", Permission::VIEW)
                    .with_description("Kanban with project rollups"),
                PageManifest::simple(page::PROJECT_LIST, "Project List", Permission::VIEW)
                    .with_description("Sortable table of all projects"),
                PageManifest::simple(page::PROJECT_SUMMARY, "Project Summary", Permission::VIEW)
                    .with_description("Read-only project detail"),
                PageManifest::simple(page::PROPOSAL_EDITOR, "Proposal Editor", Permission::EDIT)
                    .with_description("Author and revise proposals"),
                PageManifest::simple(page::PROJECT_METRICS, "Project Metrics", Permission::VIEW)
                    .with_description("Firm-level rollups"),
            ]
        })
        .as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roster_matches_manifests() {
        let m = ProjectManagementModule::new();
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
        }
    }

    #[test]
    fn top_bar_actions_cover_every_factory_button() {
        let m = ProjectManagementModule::new();
        let actions = m.top_bar_actions();
        for ws in m.factory_workspaces() {
            let Some(bar) = ws.top_bar else { continue };
            for slot in bar.slots {
                if let ses_shell::TopBarSlotKind::Button { action_id, .. } = slot.kind {
                    assert!(
                        actions.contains(&action_id.as_str()),
                        "button `{action_id}` not declared in top_bar_actions()"
                    );
                }
            }
        }
    }
}
