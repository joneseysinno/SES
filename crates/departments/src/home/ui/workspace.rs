use super::page;
use crate::home::MODULE_ID_STR;
use ses_shell::{Axis, PageDescriptor, PageNode, WorkspaceDef};

/// The Home dashboard — a projects summary beside a personal timecard.
/// Deliberately **not** `.for_department()` — the page picker stays open to
/// every department the user has permission for. No top bar in this pass;
/// `top_bar: None` is a supported state and Home proves it.
pub fn dashboard() -> WorkspaceDef {
    let root = PageNode::split(
        Axis::Horizontal,
        0.62,
        PageNode::leaf(PageDescriptor::new(MODULE_ID_STR, page::PROJECTS_OVERVIEW)),
        PageNode::leaf(PageDescriptor::new(MODULE_ID_STR, page::TIMECARD)),
    );

    WorkspaceDef::new("Home", root)
        .with_seed_key("home/dashboard")
        .with_seed_order(crate::seed_order::HOME)
}

/// Every factory workspace this department seeds at startup.
pub fn all() -> Vec<WorkspaceDef> {
    vec![dashboard()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ses_shell::PageFilter;

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
    fn dashboard_seed_key_stable() {
        assert_eq!(dashboard().seed_key.as_deref(), Some("home/dashboard"));
    }

    #[test]
    fn dashboard_is_user_defined() {
        assert_eq!(dashboard().page_filter, PageFilter::UserDefined);
    }

    #[test]
    fn dashboard_has_no_top_bar() {
        assert!(dashboard().top_bar.is_none());
    }

    #[test]
    fn dashboard_seed_order_matches_constant() {
        assert_eq!(dashboard().seed_order, crate::seed_order::HOME);
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
