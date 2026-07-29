//! Landmark definitions — named scroll-bar anchors on page leaves.

use crate::ids::{next_u64, LeafId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Opaque ID for a landmark or landmark group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LandmarkId(pub u64);

impl LandmarkId {
    pub fn new() -> Self {
        Self(next_u64())
    }
}

impl Default for LandmarkId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LandmarkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lm-{}", self.0)
    }
}

/// Visual representation of a landmark on the scroll bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandmarkIcon {
    /// Short label shown inside the icon (1–3 chars, emoji OK).
    pub label: String,
    /// Optional CSS color token (e.g. "var(--ses-accent)"). None = default.
    pub color: Option<String>,
}

impl LandmarkIcon {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: None,
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// A single landmark pinned to one or more page leaves.
/// When `leaf_ids` has more than one entry this is a group landmark —
/// the scroll bar shows a bracket spanning all member leaves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandmarkDef {
    pub id: LandmarkId,
    /// Ordered list of leaf ids covered by this landmark.
    /// Single-leaf landmark → one entry. Group → multiple entries.
    pub leaf_ids: Vec<LeafId>,
    pub icon: LandmarkIcon,
    /// Tooltip shown on hover.
    pub tooltip: Option<String>,
    /// Optional keyboard shortcut index (0-based). None = no shortcut.
    /// UI maps shortcut_index 0 → Alt+1, 1 → Alt+2, etc.
    pub shortcut_index: Option<u8>,
    /// If true, clicking focuses/zooms to fit the landmark's pods.
    pub focus_on_click: bool,
}

impl LandmarkDef {
    pub fn single(leaf_id: LeafId, icon: LandmarkIcon) -> Self {
        Self {
            id: LandmarkId::new(),
            leaf_ids: vec![leaf_id],
            icon,
            tooltip: None,
            shortcut_index: None,
            focus_on_click: false,
        }
    }

    pub fn group(leaf_ids: Vec<LeafId>, icon: LandmarkIcon) -> Self {
        Self {
            id: LandmarkId::new(),
            leaf_ids,
            icon,
            tooltip: None,
            shortcut_index: None,
            focus_on_click: false,
        }
    }

    pub fn with_tooltip(mut self, tip: impl Into<String>) -> Self {
        self.tooltip = Some(tip.into());
        self
    }

    pub fn with_shortcut(mut self, index: u8) -> Self {
        self.shortcut_index = Some(index);
        self
    }

    pub fn with_focus(mut self, focus: bool) -> Self {
        self.focus_on_click = focus;
        self
    }
}
