use crate::project::ui::page;
use crate::project::ui::workspace;
use crate::project::MODULE_ID_STR;
use ses_modules::page_manifest::PageManifest;
use ses_modules::permission::Permission;
use ses_modules::SesModule;
use ses_shell::{ModuleId, PodLayout, WorkspaceDef};
use std::sync::OnceLock;

const REQUIRES: &[&'static str] = &["project_id"];

pub struct ProjectModule {
    pages: &'static [PageManifest],
}

impl ProjectModule {
    pub fn new() -> Self {
        Self {
            pages: project_pages(),
        }
    }
}

impl SesModule for ProjectModule {
    fn id(&self) -> ModuleId {
        ModuleId::new(MODULE_ID_STR)
    }

    fn display_name(&self) -> &str {
        "Project"
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
        true
    }
}

fn project_pages() -> &'static [PageManifest] {
    static PAGES: OnceLock<Vec<PageManifest>> = OnceLock::new();
    PAGES
        .get_or_init(|| {
            vec![
                PageManifest::simple(page::TASK_BOARD, "Task Board", Permission::VIEW)
                    .with_description("Detailed Kanban for project tasks")
                    .with_requires(REQUIRES)
                    .with_layout(PodLayout::Stack),
                PageManifest::simple(page::TASK_DETAIL, "Task Detail", Permission::VIEW)
                    .with_requires(REQUIRES),
                PageManifest::simple(page::PROJECT_DOCS, "Documents", Permission::VIEW)
                    .with_requires(REQUIRES),
                PageManifest::simple(
                    page::PROJECT_ANALYSIS,
                    "Analysis",
                    Permission::VIEW | Permission::ANALYZE,
                )
                .with_requires(REQUIRES),
                PageManifest::simple(
                    page::TIME_TRACKING,
                    "Time Tracking",
                    Permission::VIEW | Permission::EDIT,
                )
                .with_requires(REQUIRES),
                PageManifest::simple(page::PROJECT_TIMELINE, "Timeline", Permission::VIEW)
                    .with_requires(REQUIRES),
                PageManifest::simple(page::PROJECT_OVERVIEW, "Overview", Permission::VIEW)
                    .with_requires(REQUIRES),
            ]
        })
        .as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roster_matches_manifests() {
        let m = ProjectModule::new();
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
            assert_eq!(manifest.requires, REQUIRES);
        }
    }
}
