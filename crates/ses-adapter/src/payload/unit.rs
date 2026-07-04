use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};
use ses_engineer::{Dim, Rational, UnitSystem};

/// Unit definition row in the `units` space (ses-vocabulary §1.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unit {
    /// Display symbol.
    pub symbol: String,
    /// Long name.
    pub name: String,
    /// Dimension signature.
    pub dim: Dim,
    /// Exact ratio to the dimension pivot.
    pub ratio_to_pivot: Rational,
    /// Unit system membership.
    pub system: UnitSystem,
    /// Display decimal places.
    pub display_precision: u8,
}

impl Versioned for Unit {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Unit {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}
