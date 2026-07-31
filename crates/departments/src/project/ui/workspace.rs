use super::page;
use crate::project::MODULE_ID_STR;
use crate::shared::ProjectId;
use ses_shell::{
    Axis, ModuleId, PageDescriptor, PageNode, PageTopBar, PodLayout, TopBarSlot, TopBarSlotKind,
    WorkspaceBinding, WorkspaceDef, WorkspaceId,
};

/// The unbound Project department template workspace.
pub fn template() -> WorkspaceDef {
    let right = PageNode::split(
        Axis::Vertical,
        0.55,
        PageNode::leaf(PageDescriptor::new(MODULE_ID_STR, page::PROJECT_OVERVIEW)),
        PageNode::leaf(PageDescriptor::new(MODULE_ID_STR, page::PROJECT_TIMELINE)),
    );
    let root = PageNode::split(
        Axis::Horizontal,
        0.62,
        PageNode::leaf(
            PageDescriptor::new(MODULE_ID_STR, page::TASK_BOARD).with_layout(PodLayout::Stack),
        ),
        right,
    );

    WorkspaceDef::new("Project", root)
        .for_department(MODULE_ID_STR)
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
pub fn for_project(project_id: ProjectId, name: &str) -> WorkspaceDef {
    let mut ws = template();
    ws.name = format!("Project — {name}");
    ws.id = WorkspaceId::new();
    ws.seed_key = None;
    ws.template_of = Some(ModuleId::new(MODULE_ID_STR));
    ws.binding
        .set(WorkspaceBinding::PROJECT_ID, project_id.0.to_string());
    ws.layout.reassign_leaf_ids();
    ws
}

/// Every factory workspace this department seeds at startup.
pub fn all() -> Vec<WorkspaceDef> {
    vec![template()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::reset_id_counter;
    use ses_shell::{LeafId, PageNode};

    fn collect_descriptors<'a>(node: &'a PageNode, out: &mut Vec<&'a ses_shell::PageDescriptor>) {
        match node {
            PageNode::Leaf(leaf) => out.push(&leaf.page),
            PageNode::Split { first, second, .. } => {
                collect_descriptors(first, out);
                collect_descriptors(second, out);
            }
        }
    }

    #[test]
    fn template_seed_key_stable() {
        assert_eq!(template().seed_key.as_deref(), Some("project/template"));
    }

    #[test]
    fn workspace_pages_are_in_roster() {
        for ws in all() {
            let mut descs = Vec::new();
            collect_descriptors(&ws.layout, &mut descs);
            for d in descs {
                assert_eq!(d.module_id.as_str(), MODULE_ID_STR);
                assert!(
                    page::ALL.contains(&d.page_id.as_str()),
                    "workspace page_id `{}` missing from ALL",
                    d.page_id.as_str()
                );
            }
        }
    }

    #[test]
    fn for_project_sets_binding_and_fresh_ids() {
        reset_id_counter(100);
        let tmpl = template();
        let template_leaves: Vec<LeafId> = tmpl.layout.leaf_ids();

        let pid = ProjectId::from_raw(42);
        let inst = for_project(pid, "Clinic");

        assert_eq!(inst.binding.get("project_id"), Some("42"));
        assert!(inst.seed_key.is_none());
        assert_eq!(inst.template_of, Some(ModuleId::new(MODULE_ID_STR)));
        assert_ne!(inst.id, tmpl.id);

        let inst_leaves = inst.layout.leaf_ids();
        assert_eq!(inst_leaves.len(), template_leaves.len());
        for (a, b) in inst_leaves.iter().zip(template_leaves.iter()) {
            assert_ne!(a, b);
        }
    }
}
