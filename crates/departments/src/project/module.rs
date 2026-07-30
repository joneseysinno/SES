use crate::project::pages::{
    ProjectAnalysisPage, ProjectDocsPage, ProjectOverviewPage, ProjectTimelinePage,
    TaskBoardPage, TaskDetailPage, TimeTrackingPage,
};
use crate::shared::ProjectId;
use dioxus::prelude::*;
use ses_modules::page_manifest::PageManifest;
use ses_modules::permission::Permission;
use ses_modules::SesModule;
use ses_shell::{
    Axis, ModuleId, PageDescriptor, PageNode, PageTopBar, PodLayout, TopBarSlot, TopBarSlotKind,
    WorkspaceBinding, WorkspaceDef, WorkspaceId,
};
use ses_ui::{PageCtx, SesModuleUi};
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
        ModuleId::new("project")
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
        vec![project_template()]
    }

    fn is_template(&self) -> bool {
        true
    }
}

pub struct ProjectUi;

impl SesModuleUi for ProjectUi {
    fn module_id(&self) -> ModuleId {
        ModuleId::new("project")
    }

    fn render_page(&self, page_id: &ses_shell::PageId, ctx: &PageCtx) -> Element {
        match page_id.as_str() {
            "task-board" => rsx! { TaskBoardPage { ctx: ctx.clone() } },
            "task-detail" => rsx! { TaskDetailPage { ctx: ctx.clone() } },
            "project-docs" => rsx! { ProjectDocsPage { ctx: ctx.clone() } },
            "project-analysis" => rsx! { ProjectAnalysisPage { ctx: ctx.clone() } },
            "time-tracking" => rsx! { TimeTrackingPage { ctx: ctx.clone() } },
            "project-timeline" => rsx! { ProjectTimelinePage { ctx: ctx.clone() } },
            "project-overview" => rsx! { ProjectOverviewPage { ctx: ctx.clone() } },
            other => rsx! {
                div { class: "ses-pod",
                    p { class: "ses-muted", "Unknown project page: {other}" }
                }
            },
        }
    }
}

fn project_pages() -> &'static [PageManifest] {
    static PAGES: OnceLock<Vec<PageManifest>> = OnceLock::new();
    PAGES
        .get_or_init(|| {
            vec![
                PageManifest::simple("task-board", "Task Board", Permission::VIEW)
                    .with_description("Detailed Kanban for project tasks")
                    .with_requires(REQUIRES)
                    .with_layout(PodLayout::Stack),
                PageManifest::simple("task-detail", "Task Detail", Permission::VIEW)
                    .with_requires(REQUIRES),
                PageManifest::simple("project-docs", "Documents", Permission::VIEW)
                    .with_requires(REQUIRES),
                PageManifest::simple("project-analysis", "Analysis", Permission::VIEW | Permission::ANALYZE)
                    .with_requires(REQUIRES),
                PageManifest::simple("time-tracking", "Time Tracking", Permission::VIEW | Permission::EDIT)
                    .with_requires(REQUIRES),
                PageManifest::simple("project-timeline", "Timeline", Permission::VIEW)
                    .with_requires(REQUIRES),
                PageManifest::simple("project-overview", "Overview", Permission::VIEW)
                    .with_requires(REQUIRES),
            ]
        })
        .as_slice()
}

fn project_template() -> WorkspaceDef {
    let right = PageNode::split(
        Axis::Vertical,
        0.55,
        PageNode::leaf(PageDescriptor::new("project", "project-overview")),
        PageNode::leaf(PageDescriptor::new("project", "project-timeline")),
    );
    let root = PageNode::split(
        Axis::Horizontal,
        0.62,
        PageNode::leaf(
            PageDescriptor::new("project", "task-board").with_layout(PodLayout::Stack),
        ),
        right,
    );

    WorkspaceDef::new("Project", root)
        .for_department("project")
        .with_seed_key("project/template")
        .with_top_bar(
            PageTopBar::new()
                .with_slot(TopBarSlot::left(TopBarSlotKind::Label {
                    text: "{project.name}".into(),
                }))
                .with_slot(TopBarSlot::right(TopBarSlotKind::FlowDisplay {
                    channel: "project.progress".into(),
                    show_channel_name: true,
                })),
        )
}

/// Clone the template workspace for one project instance.
pub fn instantiate_for(project_id: ProjectId, name: &str) -> WorkspaceDef {
    let mut ws = project_template();
    ws.name = format!("Project — {name}");
    ws.id = WorkspaceId::new();
    ws.seed_key = None;
    ws.template_of = Some(ModuleId::new("project"));
    ws.binding
        .set(WorkspaceBinding::PROJECT_ID, project_id.0.to_string());
    ws.layout.reassign_leaf_ids();
    ws
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::reset_id_counter;
    use ses_shell::LeafId;

    #[test]
    fn instantiate_for_sets_binding_and_fresh_ids() {
        reset_id_counter(100);
        let template = project_template();
        let template_leaves: Vec<LeafId> = template.layout.leaf_ids();

        let pid = ProjectId::from_raw(42);
        let inst = instantiate_for(pid, "Clinic");

        assert_eq!(inst.binding.get("project_id"), Some("42"));
        assert!(inst.seed_key.is_none());
        assert_eq!(inst.template_of, Some(ModuleId::new("project")));
        assert_ne!(inst.id, template.id);

        let inst_leaves = inst.layout.leaf_ids();
        assert_eq!(inst_leaves.len(), template_leaves.len());
        for (a, b) in inst_leaves.iter().zip(template_leaves.iter()) {
            assert_ne!(a, b);
        }
    }
}
