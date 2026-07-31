use super::page;
use crate::project_management::MODULE_ID_STR;
use ses_shell::{PageDescriptor, PageNode, PageTopBar, TopBarSlot, TopBarSlotKind, WorkspaceDef};

/// The firm-level Project Management portfolio workspace — one page, two
/// pods (progress summary + active project list).
pub fn portfolio() -> WorkspaceDef {
    let root = PageNode::leaf(PageDescriptor::new(MODULE_ID_STR, page::PORTFOLIO_OVERVIEW));

    WorkspaceDef::new("Project Management", root)
        .for_department(MODULE_ID_STR)
        .with_seed_key("project-mgmt/main")
        .with_seed_order(crate::seed_order::PROJECT_MANAGEMENT)
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

/// Every factory workspace this department seeds at startup.
pub fn all() -> Vec<WorkspaceDef> {
    vec![portfolio()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ses_shell::PageNode;

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
    fn portfolio_seed_key_stable() {
        assert_eq!(portfolio().seed_key.as_deref(), Some("project-mgmt/main"));
    }

    #[test]
    fn portfolio_seed_order_matches_constant() {
        assert_eq!(portfolio().seed_order, crate::seed_order::PROJECT_MANAGEMENT);
    }

    #[test]
    fn portfolio_is_single_leaf() {
        assert!(matches!(portfolio().layout, PageNode::Leaf(_)));
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
}
