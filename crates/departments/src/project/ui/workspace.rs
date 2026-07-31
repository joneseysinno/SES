use super::page;
use crate::project::MODULE_ID_STR;
use crate::shared::ProjectId;
use ses_shell::{
    Axis, ModuleId, PageDescriptor, PageNode, PageTopBar, PodLayout, TopBarSlot, TopBarSlotKind,
    WorkspaceBinding, WorkspaceDef, WorkspaceId,
};

/// The unbound Project department template workspace. Never seeded as a tab
/// — see [`SesModule::template_workspace`](ses_modules::SesModule::template_workspace).
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
                    text: "{project_number} · {project_name}".into(),
                }))
                .with_slot(TopBarSlot::right(TopBarSlotKind::FlowDisplay {
                    channel: "project.progress".into(),
                    show_channel_name: true,
                })),
        )
}

/// Clone the template workspace for one project instance. The tab is labeled
/// with the project number and name, and its top bar is interpolated so the
/// literal `{project_number} · {project_name}` never renders.
pub fn for_project(project_id: ProjectId, number: &str, name: &str) -> WorkspaceDef {
    let mut ws = template();
    ws.name = format!("{number} · {name}");
    ws.id = WorkspaceId::new();
    ws.seed_key = None;
    ws.seed_order = 0;
    ws.template_of = Some(ModuleId::new(MODULE_ID_STR));
    ws.binding
        .set(WorkspaceBinding::PROJECT_ID, project_id.0.to_string());
    ws.binding.set(WorkspaceBinding::PROJECT_NUMBER, number);
    ws.binding.set(WorkspaceBinding::PROJECT_NAME, name);
    ws.layout.reassign_leaf_ids();
    if let Some(bar) = ws.top_bar.as_mut() {
        ses_shell::interpolate_top_bar(bar, &ws.binding);
    }
    ws
}

/// Template modules seed no factory workspaces — a template is not a tab.
pub fn all() -> Vec<WorkspaceDef> {
    Vec::new()
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
    fn all_seeds_no_workspaces() {
        assert!(all().is_empty());
    }

    #[test]
    fn workspace_pages_are_in_roster() {
        for ws in [template()] {
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
        let inst = for_project(pid, "2026-007", "Clinic");

        assert_eq!(inst.binding.get("project_id"), Some("42"));
        assert_eq!(inst.binding.get("project_number"), Some("2026-007"));
        assert_eq!(inst.binding.get("project_name"), Some("Clinic"));
        assert_eq!(inst.name, "2026-007 · Clinic");
        assert!(inst.seed_key.is_none());
        assert_eq!(inst.template_of, Some(ModuleId::new(MODULE_ID_STR)));
        assert_ne!(inst.id, tmpl.id);

        let inst_leaves = inst.layout.leaf_ids();
        assert_eq!(inst_leaves.len(), template_leaves.len());
        for (a, b) in inst_leaves.iter().zip(template_leaves.iter()) {
            assert_ne!(a, b);
        }
    }

    #[test]
    fn for_project_interpolates_top_bar_label() {
        let pid = ProjectId::from_raw(1);
        let inst = for_project(pid, "2026-007", "Clinic");
        let bar = inst.top_bar.expect("template has a top bar");
        match &bar.slots[0].kind {
            TopBarSlotKind::Label { text } => assert_eq!(text, "2026-007 · Clinic"),
            other => panic!("expected label, got {other:?}"),
        }
    }
}
