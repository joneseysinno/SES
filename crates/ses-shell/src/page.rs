//! Page split tree — Blender-style area layout.
//!
//! A leaf holds a [`PageDescriptor`] (department-owned screen region).
//! Pods are furniture *inside* a page, not split-tree units.

use crate::ids::{LeafId, ModuleId};
use crate::pod::PodLayout;
use serde::{Deserialize, Serialize};

/// Identifies which department page occupies a leaf of the split tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageId(pub String);

impl PageId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PageId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A page is a department-owned screen region hosted in a split-tree leaf.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageDescriptor {
    /// Which department module owns and renders this page.
    pub module_id: ModuleId,
    /// Which page within that module (e.g. "project-list", "task-board").
    pub page_id: PageId,
    /// Display override for the page header strip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// How this page arranges its pods.
    #[serde(default)]
    pub pod_layout: PodLayout,
}

impl PageDescriptor {
    pub fn new(module_id: impl Into<ModuleId>, page_id: impl Into<PageId>) -> Self {
        Self {
            module_id: module_id.into(),
            page_id: page_id.into(),
            title: None,
            pod_layout: PodLayout::default(),
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn with_layout(mut self, l: PodLayout) -> Self {
        self.pod_layout = l;
        self
    }

    /// Title shown in the leaf header: override if set, otherwise page id.
    pub fn display_title(&self) -> &str {
        self.title
            .as_deref()
            .unwrap_or_else(|| self.page_id.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    /// Left | Right
    Horizontal,
    /// Top / Bottom
    Vertical,
}

impl Axis {
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Horizontal => "ses-axis-horizontal",
            Self::Vertical => "ses-axis-vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoPlacement {
    Below,
    Side,
}

/// Optional input/output panel layout within a page leaf.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IoLayout {
    pub show_input: bool,
    pub show_output: bool,
    pub placement: IoPlacement,
    /// Flow channel this leaf reads from / writes to (scaffolding string key).
    pub channel: Option<String>,
}

impl IoLayout {
    pub fn none() -> Self {
        Self {
            show_input: false,
            show_output: false,
            placement: IoPlacement::Below,
            channel: None,
        }
    }

    pub fn with_io(channel: impl Into<String>, placement: IoPlacement) -> Self {
        Self {
            show_input: true,
            show_output: true,
            placement,
            channel: Some(channel.into()),
        }
    }

    pub fn output_only(channel: impl Into<String>) -> Self {
        Self {
            show_input: false,
            show_output: true,
            placement: IoPlacement::Below,
            channel: Some(channel.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageLeaf {
    pub id: LeafId,
    pub page: PageDescriptor,
    pub io: IoLayout,
}

impl PageLeaf {
    pub fn new(page: PageDescriptor) -> Self {
        Self {
            id: LeafId::new(),
            page,
            io: IoLayout::none(),
        }
    }

    pub fn with_io(mut self, io: IoLayout) -> Self {
        self.io = io;
        self
    }
}

/// Binary split tree of pages (areas).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PageNode {
    Split {
        axis: Axis,
        /// Fraction of space for `first` (0.05..=0.95).
        ratio: f32,
        first: Box<PageNode>,
        second: Box<PageNode>,
    },
    Leaf(PageLeaf),
}

impl PageNode {
    pub fn leaf(page: PageDescriptor) -> Self {
        Self::Leaf(PageLeaf::new(page))
    }

    pub fn split(axis: Axis, ratio: f32, first: PageNode, second: PageNode) -> Self {
        Self::Split {
            axis,
            ratio: ratio.clamp(0.05, 0.95),
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    pub fn find_leaf(&self, id: LeafId) -> Option<&PageLeaf> {
        match self {
            Self::Leaf(leaf) if leaf.id == id => Some(leaf),
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => {
                first.find_leaf(id).or_else(|| second.find_leaf(id))
            }
        }
    }

    pub fn find_leaf_mut(&mut self, id: LeafId) -> Option<&mut PageLeaf> {
        match self {
            Self::Leaf(leaf) if leaf.id == id => Some(leaf),
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => first
                .find_leaf_mut(id)
                .or_else(|| second.find_leaf_mut(id)),
        }
    }

    pub fn leaf_ids(&self) -> Vec<LeafId> {
        let mut out = Vec::new();
        self.collect_leaf_ids(&mut out);
        out
    }

    fn collect_leaf_ids(&self, out: &mut Vec<LeafId>) {
        match self {
            Self::Leaf(leaf) => out.push(leaf.id),
            Self::Split { first, second, .. } => {
                first.collect_leaf_ids(out);
                second.collect_leaf_ids(out);
            }
        }
    }

    /// Assign fresh [`LeafId`]s to every leaf (template instantiation).
    pub fn reassign_leaf_ids(&mut self) {
        match self {
            Self::Leaf(leaf) => leaf.id = LeafId::new(),
            Self::Split { first, second, .. } => {
                first.reassign_leaf_ids();
                second.reassign_leaf_ids();
            }
        }
    }
}

/// Kind of slot that can live in the page top bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopBarSlotKind {
    /// A plain clickable button with a label. Action is identified by string
    /// so the UI layer can dispatch it (e.g. "export", "run-all").
    Button { label: String, action_id: String },

    /// A read-only text panel. Content is a static string set by the author.
    Label { text: String },

    /// A live panel bound to a flow channel. Displays the channel's current
    /// FlowValue, updating reactively. Optionally shows the channel name.
    FlowDisplay {
        channel: String,
        show_channel_name: bool,
    },

    /// A visual separator (vertical rule).
    Separator,
}

/// Alignment of a slot within the top bar flex row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopBarAlign {
    Left,
    Center,
    Right,
}

/// One slot in the page top bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopBarSlot {
    pub kind: TopBarSlotKind,
    pub align: TopBarAlign,
}

impl TopBarSlot {
    pub fn left(kind: TopBarSlotKind) -> Self {
        Self {
            kind,
            align: TopBarAlign::Left,
        }
    }

    pub fn center(kind: TopBarSlotKind) -> Self {
        Self {
            kind,
            align: TopBarAlign::Center,
        }
    }

    pub fn right(kind: TopBarSlotKind) -> Self {
        Self {
            kind,
            align: TopBarAlign::Right,
        }
    }
}

/// Discrete height sizes for the page top bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopBarHeight {
    /// 28px
    Compact,
    /// 36px
    Standard,
    /// 52px
    Tall,
}

impl TopBarHeight {
    pub fn px(self) -> u32 {
        match self {
            Self::Compact => 28,
            Self::Standard => 36,
            Self::Tall => 52,
        }
    }
}

/// Optional sticky top bar for a workspace page area.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageTopBar {
    pub visible: bool,
    pub height: TopBarHeight,
    pub slots: Vec<TopBarSlot>,
}

impl PageTopBar {
    pub fn new() -> Self {
        Self {
            visible: true,
            height: TopBarHeight::Standard,
            slots: Vec::new(),
        }
    }

    pub fn with_slot(mut self, slot: TopBarSlot) -> Self {
        self.slots.push(slot);
        self
    }
}

impl Default for PageTopBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Substitute `{key}` in Label slots from the workspace binding.
/// Unknown keys are left verbatim so authoring mistakes stay visible
/// instead of silently rendering an empty bar.
pub fn interpolate_top_bar(bar: &mut PageTopBar, binding: &crate::workspace::WorkspaceBinding) {
    for slot in &mut bar.slots {
        if let TopBarSlotKind::Label { text } = &mut slot.kind {
            *text = interpolate_binding_text(text, binding);
        }
    }
}

fn interpolate_binding_text(input: &str, binding: &crate::workspace::WorkspaceBinding) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        if let Some(end) = after.find('}') {
            let key = &after[..end];
            if let Some(val) = binding.get(key) {
                out.push_str(val);
            } else {
                out.push('{');
                out.push_str(key);
                out.push('}');
            }
            rest = &after[end + 1..];
        } else {
            out.push('{');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod top_bar_tests {
    use super::{
        PageTopBar, TopBarHeight, TopBarSlot, TopBarSlotKind, interpolate_top_bar,
    };
    use crate::workspace::WorkspaceBinding;

    #[test]
    fn top_bar_height_px() {
        assert_eq!(TopBarHeight::Compact.px(), 28);
        assert_eq!(TopBarHeight::Standard.px(), 36);
        assert_eq!(TopBarHeight::Tall.px(), 52);
    }

    #[test]
    fn interpolate_known_and_unknown_keys() {
        let mut bar = PageTopBar::new()
            .with_slot(TopBarSlot::left(TopBarSlotKind::Label {
                text: "{project_number} · {project_name}".into(),
            }))
            .with_slot(TopBarSlot::right(TopBarSlotKind::Button {
                label: "Go".into(),
                action_id: "go".into(),
            }))
            .with_slot(TopBarSlot::right(TopBarSlotKind::Label {
                text: "{missing}".into(),
            }));
        let mut binding = WorkspaceBinding::default();
        binding.set("project_number", "2026-001");
        binding.set("project_name", "Clinic");
        interpolate_top_bar(&mut bar, &binding);
        match &bar.slots[0].kind {
            TopBarSlotKind::Label { text } => assert_eq!(text, "2026-001 · Clinic"),
            other => panic!("expected label, got {other:?}"),
        }
        match &bar.slots[1].kind {
            TopBarSlotKind::Button { label, .. } => assert_eq!(label, "Go"),
            other => panic!("expected button, got {other:?}"),
        }
        match &bar.slots[2].kind {
            TopBarSlotKind::Label { text } => assert_eq!(text, "{missing}"),
            other => panic!("expected label, got {other:?}"),
        }
    }

    #[test]
    fn interpolate_no_binding_leaves_text() {
        let mut bar = PageTopBar::new().with_slot(TopBarSlot::left(TopBarSlotKind::Label {
            text: "{project_name}".into(),
        }));
        interpolate_top_bar(&mut bar, &WorkspaceBinding::default());
        match &bar.slots[0].kind {
            TopBarSlotKind::Label { text } => assert_eq!(text, "{project_name}"),
            other => panic!("expected label, got {other:?}"),
        }
    }
}
