use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

/// Default role for an adopting code entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CodeRole {
    /// Primary adopting code.
    #[default]
    Adopting,
    /// Loads standard.
    Loads,
    /// Material design standard.
    MaterialDesign,
    /// Prescriptive provisions.
    Prescriptive,
}

/// Adopted code record in the `codes` space (ses-vocabulary §2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Code {
    /// Code family label.
    pub family: String,
    /// Edition label.
    pub edition_label: String,
    /// Role in the code stack.
    pub role: CodeRole,
    /// Issuing body.
    pub issuing_body: String,
    /// Effective-from timestamp (UTC seconds), if known.
    pub effective_from_utc: Option<i64>,
    /// Effective-to timestamp (UTC seconds), if known.
    pub effective_to_utc: Option<i64>,
}

impl Versioned for Code {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Code {
    const KIND: TestimonyKind = TestimonyKind::Imported;
    const WITNESSES: &'static [&'static str] = &["catalog"];
}
