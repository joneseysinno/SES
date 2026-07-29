//! Pod descriptors and kinds.

use crate::ids::ModuleId;
use serde::{Deserialize, Serialize};

/// Functional editor type hosted inside a page leaf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PodKind {
    TopBar,
    StatusBar,
    MenuBar,
    View,
    Outliner,
    Properties,
    Calculation,
}

impl PodKind {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::TopBar => "Top Bar",
            Self::StatusBar => "Status Bar",
            Self::MenuBar => "Menu Bar",
            Self::View => "3D Viewport",
            Self::Outliner => "Outliner",
            Self::Properties => "Properties",
            Self::Calculation => "Calculation",
        }
    }

    /// Pod kinds selectable in a page leaf header.
    pub fn page_kinds() -> &'static [PodKind] {
        &[
            Self::View,
            Self::Outliner,
            Self::Properties,
            Self::Calculation,
            Self::MenuBar,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PodDescriptor {
    pub kind: PodKind,
    pub module_id: ModuleId,
    /// Optional display override for the leaf title strip (e.g. "Geometry").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// When true, the leaf header shows a collapse toggle.
    #[serde(default)]
    pub collapsible: bool,
    /// When true (and collapsible), only the title strip is shown.
    #[serde(default)]
    pub collapsed: bool,
}

impl PodDescriptor {
    pub fn new(kind: PodKind, module_id: impl Into<ModuleId>) -> Self {
        Self {
            kind,
            module_id: module_id.into(),
            title: None,
            collapsible: false,
            collapsed: false,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Mark as collapsible and start collapsed (title strip only).
    pub fn start_collapsed(mut self) -> Self {
        self.collapsible = true;
        self.collapsed = true;
        self
    }

    /// Title shown in the leaf header: override if set, otherwise kind name.
    pub fn display_title(&self) -> &str {
        self.title
            .as_deref()
            .unwrap_or_else(|| self.kind.display_name())
    }
}
