//! Page split tree — Blender-style area layout.

use crate::ids::LeafId;
use crate::pod::PodDescriptor;
use serde::{Deserialize, Serialize};

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
    pub pod: PodDescriptor,
    pub io: IoLayout,
}

impl PageLeaf {
    pub fn new(pod: PodDescriptor) -> Self {
        Self {
            id: LeafId::new(),
            pod,
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
    pub fn leaf(pod: PodDescriptor) -> Self {
        Self::Leaf(PageLeaf::new(pod))
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

#[cfg(test)]
mod top_bar_tests {
    use super::TopBarHeight;

    #[test]
    fn top_bar_height_px() {
        assert_eq!(TopBarHeight::Compact.px(), 28);
        assert_eq!(TopBarHeight::Standard.px(), 36);
        assert_eq!(TopBarHeight::Tall.px(), 52);
    }
}
