//! Default workspace layouts seeded on first launch.

use crate::ids::{ModuleId, reset_id_counter};
use crate::page::{Axis, IoLayout, IoPlacement, PageLeaf, PageNode};
use crate::pod::{PodDescriptor, PodKind};
use crate::workspace::{ShellState, WorkspaceDef};

fn pod(kind: PodKind, module: &str) -> PodDescriptor {
    PodDescriptor::new(kind, ModuleId::new(module))
}

fn leaf(kind: PodKind, module: &str) -> PageNode {
    PageNode::Leaf(PageLeaf::new(pod(kind, module)))
}

fn leaf_with_io(kind: PodKind, module: &str, io: IoLayout) -> PageNode {
    PageNode::Leaf(PageLeaf::new(pod(kind, module)).with_io(io))
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
        leaf_with_io(
            PodKind::Calculation,
            "analysis",
            IoLayout::output_only("calc.result"),
        ),
    );
    let root = PageNode::split(Axis::Vertical, 0.7, top, bottom);
    WorkspaceDef::new("Layout", root)
}

/// Analysis: View (wide left) + Calculation with I/O (right)
fn analysis_workspace() -> WorkspaceDef {
    let calc = leaf_with_io(
        PodKind::Calculation,
        "analysis",
        IoLayout::with_io("calc.result", IoPlacement::Below),
    );
    let root = PageNode::split(Axis::Horizontal, 0.7, leaf(PodKind::View, "core-ui"), calc);
    WorkspaceDef::new("Analysis", root)
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
        assert!(shell.active().is_some());
    }
}
