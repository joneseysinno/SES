//! Page manifests — pages a department offers to the page picker.

use crate::permission::Permission;
use ses_shell::{PageId, PodLayout};

/// Declaration of one page a department offers.
#[derive(Debug, Clone, PartialEq)]
pub struct PageManifest {
    pub page_id: PageId,
    pub display_name: &'static str,
    /// Shown in the page picker's grouped list.
    pub description: &'static str,
    /// Binding keys this page requires. A page listing "project_id" will not
    /// appear in the picker for a workspace that has no project bound.
    pub requires: &'static [&'static str],
    /// Default pod layout when this page is first placed.
    pub default_layout: PodLayout,
    pub permission: Permission,
}

impl PageManifest {
    pub fn simple(
        page_id: &'static str,
        display_name: &'static str,
        permission: Permission,
    ) -> Self {
        Self {
            page_id: PageId::new(page_id),
            display_name,
            description: "",
            requires: &[],
            default_layout: PodLayout::Stack,
            permission,
        }
    }

    pub fn with_requires(mut self, requires: &'static [&'static str]) -> Self {
        self.requires = requires;
        self
    }

    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn with_layout(mut self, layout: PodLayout) -> Self {
        self.default_layout = layout;
        self
    }
}
