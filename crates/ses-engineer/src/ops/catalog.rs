use super::EmpiricalOp;
use crate::Dim;
use crate::expr::TowerLevel;

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
