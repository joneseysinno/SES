use ses_engineer::expr::{Expr, Predicate, TowerLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProvisionKind {
    Routing,
    Parameter,
    Procedure,
    Requirement,
    Detailing,
    Modification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModificationMode {
    Replace,
    Supplement,
    Except,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisionBlock {
    pub id: String,
    pub kind: ProvisionKind,
    pub title: String,
    pub applies_when: Option<Predicate>,
    pub narrative: String,
    pub tower_level: TowerLevel,
    pub precedence: i32,
    pub modifies: Vec<(String, ModificationMode)>,
    pub requires: Vec<String>,
    pub expression: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledProvision {
    pub block: ProvisionBlock,
    pub ast_hash: String,
}
