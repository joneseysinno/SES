use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};
use ses_engineer::Quantity;

/// Material property bundle (ses-vocabulary §2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialKind {
    /// Normal-weight concrete.
    Concrete {
        /// Compressive strength `f'c`.
        fc: Quantity,
        /// Lightweight factor λ.
        lambda: Quantity,
        /// Unit weight.
        wc: Quantity,
    },
    /// Reinforcing steel.
    Rebar {
        /// Yield strength.
        fy: Quantity,
        /// Ultimate strength.
        fu: Quantity,
        /// Grade label.
        grade: String,
    },
}

/// Material record in the `materials` space.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Material {
    /// Property bundle.
    pub kind: MaterialKind,
    /// Display label.
    pub label: String,
}

impl Versioned for Material {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Material {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}
