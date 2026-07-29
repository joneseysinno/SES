//! Default workspace layouts seeded on first launch.

use crate::ids::{ModuleId, reset_id_counter};
use crate::page::{
    Axis, IoLayout, IoPlacement, PageLeaf, PageNode, PageTopBar, TopBarSlot, TopBarSlotKind,
};
use crate::pod::{PodDescriptor, PodKind};
use crate::workspace::{ShellState, WorkspaceDef};

fn pod(kind: PodKind, module: &str) -> PodDescriptor {
    PodDescriptor::new(kind, ModuleId::new(module))
}

fn leaf(kind: PodKind, module: &str) -> PageNode {
    PageNode::Leaf(PageLeaf::new(pod(kind, module)))
}

fn leaf_from(descriptor: PodDescriptor) -> PageNode {
    PageNode::Leaf(PageLeaf::new(descriptor))
}

fn leaf_from_with_io(descriptor: PodDescriptor, io: IoLayout) -> PageNode {
    PageNode::Leaf(PageLeaf::new(descriptor).with_io(io))
}

/// Layout workspace: 2×2 grid — View | Outliner / Properties | Calculation
fn layout_workspace() -> WorkspaceDef {
    let top = PageNode::split(
        Axis::Horizontal,
        0.65,
        leaf(PodKind::View, "core-ui"),
        leaf(PodKind::Outliner, "core-ui"),
    );
    let bottom = PageNode::split(
        Axis::Horizontal,
        0.5,
        leaf(PodKind::Properties, "core-ui"),
        leaf_from_with_io(
            pod(PodKind::Calculation, "analysis")
                .with_title("Inputs")
                .collapsible(),
            IoLayout::output_only("calc.result"),
        ),
    );
    let root = PageNode::split(Axis::Vertical, 0.7, top, bottom);
    // Clean default — no page top bar.
    WorkspaceDef::new("Layout", root)
}

/// Analysis: View (wide left) + stacked Inputs (expanded) / Checks (collapsed)
fn analysis_workspace() -> WorkspaceDef {
    let inputs = leaf_from_with_io(
        pod(PodKind::Calculation, "analysis")
            .with_title("Inputs")
            .collapsible(),
        IoLayout::with_io("calc.result", IoPlacement::Below),
    );
    let checks = leaf_from(
        pod(PodKind::Calculation, "analysis")
            .with_title("Checks")
            .start_collapsed(),
    );
    let right = PageNode::split(Axis::Vertical, 0.72, inputs, checks);
    let root = PageNode::split(Axis::Horizontal, 0.7, leaf(PodKind::View, "core-ui"), right);
    WorkspaceDef::new("Analysis", root).with_top_bar(
        PageTopBar::new()
            .with_slot(TopBarSlot::left(TopBarSlotKind::Label {
                text: "Analysis".into(),
            }))
            .with_slot(TopBarSlot::right(TopBarSlotKind::FlowDisplay {
                channel: "calc.result".into(),
                show_channel_name: true,
            })),
    )
}

/// Documentation: Outliner + Properties
fn documentation_workspace() -> WorkspaceDef {
    let root = PageNode::split(
        Axis::Horizontal,
        0.4,
        leaf(PodKind::Outliner, "documentation"),
        leaf(PodKind::Properties, "documentation"),
    );
    WorkspaceDef::new("Documentation", root)
}

/// Seed the default shell state (three workspaces, Layout active).
pub fn default_shell() -> ShellState {
    reset_id_counter(1);
    let layout = layout_workspace();
    let active = layout.id;
    ShellState {
        workspaces: vec![layout, analysis_workspace(), documentation_workspace()],
        active_workspace: active,
        status_message: "Ready".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_three_workspaces() {
        let shell = default_shell();
        assert_eq!(shell.workspaces.len(), 3);
        assert_eq!(shell.workspaces[0].name, "Layout");
        assert!(shell.workspaces[0].top_bar.is_none());
        assert!(shell.workspaces[1].top_bar.is_some());
        assert!(shell.active().is_some());
    }

    #[test]
    fn analysis_has_collapsed_checks_pod() {
        let shell = default_shell();
        let analysis = &shell.workspaces[1];
        let titles: Vec<_> = analysis
            .layout
            .leaf_ids()
            .into_iter()
            .filter_map(|id| analysis.layout.find_leaf(id))
            .filter_map(|leaf| leaf.pod.title.as_deref())
            .collect();
        assert!(titles.contains(&"Inputs"));
        assert!(titles.contains(&"Checks"));

        let checks = analysis
            .layout
            .leaf_ids()
            .into_iter()
            .filter_map(|id| analysis.layout.find_leaf(id))
            .find(|leaf| leaf.pod.title.as_deref() == Some("Checks"))
            .expect("Checks leaf");
        assert!(checks.pod.collapsible);
        assert!(checks.pod.collapsed);
    }
}
