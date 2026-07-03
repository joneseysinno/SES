use serde::{Deserialize, Serialize};
use ses_engineer::expr::{Expr, Predicate, TowerLevel};

use crate::codec::SesPayload;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProvisionKind {
    #[default]
    Routing,
    Parameter,
    Procedure,
    Requirement,
    Detailing,
    Modification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Provision {
    pub code_addr: [u64; 6],
    pub section_string: String,
    pub title: String,
    pub summary: String,
    pub kind: ProvisionKind,
    pub predicate: Option<Predicate>,
    pub expression: Option<Expr>,
    pub tower_level: TowerLevel,
    pub unit_context: Option<String>,
    pub narrative_template: String,
    pub precedence_class: i32,
    pub parameters: Vec<(String, Expr)>,
    pub supersedes: Option<[u64; 6]>,
}

impl SesPayload for Provision {
    const SCHEMA_VERSION: u8 = 1;
}
