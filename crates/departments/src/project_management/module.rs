use crate::project_management::pages::{
    ProjectBoardPage, ProjectListPage, ProjectMetricsPage, ProjectSummaryPage, ProposalEditorPage,
};
use dioxus::prelude::*;
use ses_modules::page_manifest::PageManifest;
use ses_modules::permission::Permission;
use ses_modules::SesModule;
use ses_shell::{
    Axis, ModuleId, PageDescriptor, PageNode, PageTopBar, TopBarSlot, TopBarSlotKind, WorkspaceDef,
};
use ses_ui::{PageCtx, SesModuleUi};
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
        ModuleId::new("project-mgmt")
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
        vec![project_management_workspace()]
    }

    fn is_template(&self) -> bool {
        false
    }
}

pub struct ProjectManagementUi;

impl SesModuleUi for ProjectManagementUi {
    fn module_id(&self) -> ModuleId {
        ModuleId::new("project-mgmt")
    }

    fn render_page(&self, page_id: &ses_shell::PageId, ctx: &PageCtx) -> Element {
        match page_id.as_str() {
            "project-board" | "portfolio-board" => rsx! { ProjectBoardPage { ctx: ctx.clone() } },
            "project-list" => rsx! { ProjectListPage { ctx: ctx.clone() } },
            "project-summary" => rsx! { ProjectSummaryPage { ctx: ctx.clone() } },
            "proposal-editor" => rsx! { ProposalEditorPage { ctx: ctx.clone() } },
            "project-metrics" | "portfolio-metrics" => {
                rsx! { ProjectMetricsPage { ctx: ctx.clone() } }
            }
            other => rsx! {
                div { class: "ses-pod",
                    p { class: "ses-muted", "Unknown project-mgmt page: {other}" }
                }
            },
        }
    }
}

fn mgmt_pages() -> &'static [PageManifest] {
    static PAGES: OnceLock<Vec<PageManifest>> = OnceLock::new();
    PAGES
        .get_or_init(|| {
            vec![
                PageManifest::simple("project-board", "Project Board", Permission::VIEW)
                    .with_description("Kanban with project rollups"),
                PageManifest::simple("project-list", "Project List", Permission::VIEW)
                    .with_description("Sortable table of all projects"),
                PageManifest::simple("project-summary", "Project Summary", Permission::VIEW)
                    .with_description("Read-only project detail"),
                PageManifest::simple("proposal-editor", "Proposal Editor", Permission::EDIT)
                    .with_description("Author and revise proposals"),
                PageManifest::simple("project-metrics", "Project Metrics", Permission::VIEW)
                    .with_description("Firm-level rollups"),
            ]
        })
        .as_slice()
}

fn project_management_workspace() -> WorkspaceDef {
    let root = PageNode::split(
        Axis::Horizontal,
        0.7,
        PageNode::leaf(PageDescriptor::new("project-mgmt", "project-board")),
        PageNode::leaf(PageDescriptor::new("project-mgmt", "project-list")),
    );

    WorkspaceDef::new("Project Management", root)
        .for_department("project-mgmt")
        .with_seed_key("project-mgmt/main")
        .with_top_bar(
            PageTopBar::new()
                .with_slot(TopBarSlot::right(TopBarSlotKind::Button {
                    label: "New Project".into(),
                    action_id: "new-project".into(),
                }))
                .with_slot(TopBarSlot::right(TopBarSlotKind::Button {
                    label: "New Proposal".into(),
                    action_id: "new-proposal".into(),
                })),
        )
}
