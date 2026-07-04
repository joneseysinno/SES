use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};
use ses_engineer::Quantity;

/// Special structural wall geometry parameters (ses-vocabulary §2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialWallGeometry {
    /// Wall length.
    pub lw: Quantity,
    /// Wall height.
    pub hw: Quantity,
    /// Wall thickness.
    pub tw: Quantity,
    /// Story heights bottom to top.
    pub story_heights: Vec<Quantity>,
    /// Number of curtain segments.
    pub n_curtains: u8,
}

/// Structural element kind discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementKind {
    /// Special reinforced concrete wall.
    SpecialWall(SpecialWallGeometry),
}

/// Structural element in the `elements` space (ses-vocabulary §2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Element {
    /// Element kind and parameters.
    pub kind: ElementKind,
    /// Display label.
    pub label: String,
    /// Grid location string.
    pub grid_location: String,
    /// Free-form notes.
    pub notes: String,
}

impl Versioned for Element {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Element {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}
