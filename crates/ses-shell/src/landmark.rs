//! Landmark definitions — named scroll-bar anchors on pods within pages.

use crate::ids::{next_u64, LeafId, PodId};
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

/// A pod within a leaf that a landmark can target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LandmarkAnchor {
    pub leaf_id: LeafId,
    pub pod_id: PodId,
}

impl LandmarkAnchor {
    pub fn new(leaf_id: LeafId, pod_id: PodId) -> Self {
        Self { leaf_id, pod_id }
    }
}

/// A single landmark pinned to one or more pods.
/// When `anchors` has more than one entry this is a group landmark —
/// the scroll bar shows a bracket spanning all member leaves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandmarkDef {
    pub id: LandmarkId,
    /// Ordered list of (leaf, pod) anchors covered by this landmark.
    pub anchors: Vec<LandmarkAnchor>,
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
    pub fn single(leaf_id: LeafId, pod_id: PodId, icon: LandmarkIcon) -> Self {
        Self {
            id: LandmarkId::new(),
            anchors: vec![LandmarkAnchor::new(leaf_id, pod_id)],
            icon,
            tooltip: None,
            shortcut_index: None,
            focus_on_click: false,
        }
    }

    pub fn group(anchors: Vec<LandmarkAnchor>, icon: LandmarkIcon) -> Self {
        Self {
            id: LandmarkId::new(),
            anchors,
            icon,
            tooltip: None,
            shortcut_index: None,
            focus_on_click: false,
        }
    }

    pub fn leaf_ids(&self) -> Vec<LeafId> {
        let mut out = Vec::new();
        for a in &self.anchors {
            if !out.contains(&a.leaf_id) {
                out.push(a.leaf_id);
            }
        }
        out
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
