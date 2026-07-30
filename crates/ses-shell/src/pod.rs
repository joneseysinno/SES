//! Pod descriptors — collapsible sections inside a page.
//!
//! Pods hold IO components. They never split. `PodKind` names chrome / reflow
//! behavior, never content.

use crate::ids::PodId;
use serde::{Deserialize, Serialize};

/// Behavioral classification of a pod. Determines chrome and reflow policy —
/// NOT what the pod contains. Content is always IO components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PodKind {
    /// Standard collapsible section. Collapse toggle, landmark-eligible,
    /// reflows into a single column below the narrow breakpoint.
    Section,

    /// Always visible, no collapse toggle. Used for primary content that
    /// must never be hidden (e.g. the Kanban board itself).
    Anchor,

    /// Long scrolling content. Always landmark-eligible; gets its own
    /// sub-scroll region when the page area is tight.
    Scroller,

    /// Compact readout. Never collapses, never reflows, pinned to the top
    /// of the pod stack regardless of declared order.
    Summary,

    /// Department-defined behavior. The owning module supplies chrome rules.
    Custom,
}

impl PodKind {
    pub fn collapsible(self) -> bool {
        matches!(self, Self::Section | Self::Scroller)
    }

    pub fn landmark_eligible(self) -> bool {
        matches!(self, Self::Section | Self::Scroller | Self::Anchor)
    }

    /// Summary pods float to the top of the stack.
    pub fn sort_weight(self) -> u8 {
        match self {
            Self::Summary => 0,
            Self::Anchor => 1,
            _ => 2,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Section => "Section",
            Self::Anchor => "Anchor",
            Self::Scroller => "Scroller",
            Self::Summary => "Summary",
            Self::Custom => "Custom",
        }
    }
}

/// How a page arranges its pods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PodLayout {
    /// Vertical stack, full width. Reflows to itself at all sizes.
    Stack,
    /// N-column grid that degrades to Stack below `min_col_px * cols`.
    Grid { cols: u8, min_col_px: u32 },
}

impl Default for PodLayout {
    fn default() -> Self {
        Self::Stack
    }
}

fn one() -> u8 {
    1
}

/// A pod within a page — chrome + identity. Content is IO, not stored here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PodDescriptor {
    pub id: PodId,
    pub kind: PodKind,
    pub title: String,
    /// Landmark icon label (1–3 chars) if this pod appears on the scroll bar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub landmark_label: Option<String>,
    #[serde(default)]
    pub collapsed: bool,
    /// Grid span when the parent page uses [`PodLayout::Grid`].
    #[serde(default = "one")]
    pub col_span: u8,
}

impl PodDescriptor {
    pub fn new(kind: PodKind, title: impl Into<String>) -> Self {
        Self {
            id: PodId::new(),
            kind,
            title: title.into(),
            landmark_label: None,
            collapsed: false,
            col_span: 1,
        }
    }

    pub fn with_landmark(mut self, label: impl Into<String>) -> Self {
        self.landmark_label = Some(label.into());
        self
    }

    pub fn start_collapsed(mut self) -> Self {
        if self.kind.collapsible() {
            self.collapsed = true;
        }
        self
    }

    pub fn with_col_span(mut self, span: u8) -> Self {
        self.col_span = span.max(1);
        self
    }

    pub fn display_title(&self) -> &str {
        &self.title
    }
}
