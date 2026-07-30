use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};
use ses_engineer::Quantity;

/// Provenance of a demand record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DemandSource {
    /// Manually entered by the engineer.
    #[default]
    Manual,
    /// Imported from an external analysis.
    Imported {
        /// Origin label.
        origin: String,
    },
}

/// Load demand on an element (ses-vocabulary §2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Demand {
    /// Load combination label.
    pub combo_label: String,
    /// Factored axial load.
    pub pu: Quantity,
    /// Factored shear.
    pub vu: Quantity,
    /// Factored moment.
    pub mu: Quantity,
    /// Displacement demand.
    pub delta_u: Quantity,
    /// Story shear index.
    pub hsx: Quantity,
    /// Entry provenance.
    pub source: DemandSource,
}

impl Versioned for Demand {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Demand {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}
