//! Factory workspace seed — not authoritative. User startup profile wins.

use crate::ids::{ModuleId, reset_id_counter};
use crate::page::{
    Axis, IoLayout, IoPlacement, PageDescriptor, PageLeaf, PageNode, PageTopBar, TopBarSlot,
    TopBarSlotKind,
};
use crate::workspace::{ShellState, WorkspaceDef};

fn page(module: &str, page_id: &str) -> PageDescriptor {
    PageDescriptor::new(ModuleId::new(module), page_id)
}

fn leaf(module: &str, page_id: &str) -> PageNode {
    PageNode::Leaf(PageLeaf::new(page(module, page_id)))
}

fn leaf_titled(module: &str, page_id: &str, title: &str) -> PageNode {
    PageNode::Leaf(PageLeaf::new(page(module, page_id).with_title(title)))
}

fn leaf_with_io(module: &str, page_id: &str, title: &str, io: IoLayout) -> PageNode {
    PageNode::Leaf(PageLeaf::new(page(module, page_id).with_title(title)).with_io(io))
}

/// Minimal blank workspace used when resolve_startup would otherwise be empty.
pub fn blank_workspace() -> WorkspaceDef {
    WorkspaceDef::new("Blank", leaf("core-ui", "view")).with_seed_key("core/blank")
}

/// Layout workspace: 2×2 grid — View | Outliner / Properties | Calculation
fn layout_workspace() -> WorkspaceDef {
    let top = PageNode::split(
        Axis::Horizontal,
        0.65,
        leaf("core-ui", "view"),
        leaf("core-ui", "outliner"),
    );
    let bottom = PageNode::split(
        Axis::Horizontal,
        0.5,
        leaf("core-ui", "properties"),
        leaf_with_io(
            "analysis",
            "calculation",
            "Inputs",
            IoLayout::output_only("calc.result"),
        ),
    );
    let root = PageNode::split(Axis::Vertical, 0.7, top, bottom);
    WorkspaceDef::new("Layout", root).with_seed_key("core/layout")
}

/// Analysis: View (wide left) + stacked Inputs / Checks
fn analysis_workspace() -> WorkspaceDef {
    let inputs = leaf_with_io(
        "analysis",
        "calculation",
        "Inputs",
        IoLayout::with_io("calc.result", IoPlacement::Below),
    );
    let checks = leaf_titled("analysis", "checks", "Checks");
    let right = PageNode::split(Axis::Vertical, 0.72, inputs, checks);
    let root = PageNode::split(Axis::Horizontal, 0.7, leaf("core-ui", "view"), right);
    WorkspaceDef::new("Analysis", root)
        .with_seed_key("core/analysis")
        .with_top_bar(
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
        leaf("documentation", "outliner"),
        leaf("documentation", "properties"),
    );
    WorkspaceDef::new("Documentation", root).with_seed_key("core/documentation")
}

/// Built-in factory workspaces (core UI). Department modules contribute
/// additional seeds via their `factory_workspaces()` methods.
pub fn core_factory_workspaces() -> Vec<WorkspaceDef> {
    vec![
        layout_workspace(),
        analysis_workspace(),
        documentation_workspace(),
    ]
}

/// Seed the default shell state (three workspaces, Layout active).
/// Prefer [`crate::startup::resolve_startup`] when a user profile exists.
pub fn default_shell() -> ShellState {
    reset_id_counter(1);
    let workspaces = core_factory_workspaces();
    let active = workspaces[0].id;
    ShellState {
        workspaces,
        active_workspace: active,
        status_message: "Ready".into(),
    }
}

/// Collect factory workspaces: core seeds plus any from `extra`.
/// `enabled_modules` filters extras by module id when non-empty.
pub fn factory_workspaces(
    enabled_modules: &[ModuleId],
    extra: impl IntoIterator<Item = WorkspaceDef>,
) -> Vec<WorkspaceDef> {
    let mut out = core_factory_workspaces();
    for ws in extra {
        if enabled_modules.is_empty() {
            out.push(ws);
            continue;
        }
        let keep = match &ws.page_filter {
            crate::workspace::PageFilter::Department(m) => enabled_modules.contains(m),
            crate::workspace::PageFilter::UserDefined => true,
        };
        if keep {
            out.push(ws);
        }
    }
    out
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
        assert_eq!(
            shell.workspaces[0].seed_key.as_deref(),
            Some("core/layout")
        );
    }

    #[test]
    fn analysis_has_inputs_and_checks_pages() {
        let shell = default_shell();
        let analysis = &shell.workspaces[1];
        let titles: Vec<_> = analysis
            .layout
            .leaf_ids()
            .into_iter()
            .filter_map(|id| analysis.layout.find_leaf(id))
            .filter_map(|leaf| leaf.page.title.as_deref())
            .collect();
        assert!(titles.contains(&"Inputs"));
        assert!(titles.contains(&"Checks"));
    }
}
