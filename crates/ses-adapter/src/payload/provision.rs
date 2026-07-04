use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};
use ses_engineer::expr::{Expr, Predicate, TowerLevel};

/// Default provision kind for catalog entries without explicit classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProvisionKind {
    /// Routing provision.
    #[default]
    Routing,
    /// Parameter definition.
    Parameter,
    /// Procedure step.
    Procedure,
    /// Requirement check.
    Requirement,
    /// Detailing rule.
    Detailing,
    /// Code modification.
    Modification,
}

/// Code provision node (ses-code-pipeline §6).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provision {
    /// Address in the provisions space.
    pub code_addr: [u64; 6],
    /// Section reference string.
    pub section_string: String,
    /// Provision title.
    pub title: String,
    /// Short summary.
    pub summary: String,
    /// Provision classification.
    pub kind: ProvisionKind,
    /// Applicability predicate.
    pub predicate: Option<Predicate>,
    /// Governing expression.
    pub expression: Option<Expr>,
    /// Required tower level for evaluation.
    pub tower_level: TowerLevel,
    /// Unit context label.
    pub unit_context: Option<String>,
    /// Narrative template.
    pub narrative_template: String,
    /// Precedence ordering.
    pub precedence_class: i32,
    /// Named parameters.
    pub parameters: Vec<(String, Expr)>,
    /// Superseded provision address, if any.
    pub supersedes: Option<[u64; 6]>,
}

impl Versioned for Provision {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Provision {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}
