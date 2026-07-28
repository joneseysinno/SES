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
}

impl PodDescriptor {
    pub fn new(kind: PodKind, module_id: impl Into<ModuleId>) -> Self {
        Self {
            kind,
            module_id: module_id.into(),
        }
    }
}
