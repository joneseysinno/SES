use crate::expr::tower::TowerLevel;
use ses_core::Dim;

/// Catalogued empirical operation with declared dimension behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmpiricalOp {
    pub name: &'static str,
    pub input_dim: Dim,
    pub output_dim: Dim,
    pub tower_effect: TowerLevel,
    pub description: &'static str,
}

static CATALOG: &[EmpiricalOp] = &[
    EmpiricalOp {
        name: "sqrt_psi",
        input_dim: Dim::STRESS,
        output_dim: Dim::STRESS,
        tower_effect: TowerLevel::Algebraic,
        description: "ACI empirical root; stress in psi → stress in sqrt-psi convention",
    },
    EmpiricalOp {
        name: "interp",
        input_dim: Dim::DIMENSIONLESS,
        output_dim: Dim::DIMENSIONLESS,
        tower_effect: TowerLevel::Rational,
        description: "Linear interpolation with exact rational endpoints",
    },
    EmpiricalOp {
        name: "abs",
        input_dim: Dim::DIMENSIONLESS,
        output_dim: Dim::DIMENSIONLESS,
        tower_effect: TowerLevel::Rational,
        description: "Absolute value",
    },
];

/// v1 catalogued calls per ses-provision-dsl §3.
pub fn catalog() -> &'static [EmpiricalOp] {
    CATALOG
}

pub fn lookup(name: &str) -> Option<&'static EmpiricalOp> {
    catalog().iter().find(|op| op.name == name)
}
