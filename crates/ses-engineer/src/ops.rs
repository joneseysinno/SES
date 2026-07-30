use crate::Dim;
use crate::expr::tower::TowerLevel;

mod catalog;
mod lookup;
mod sqrt_psi;

/// Catalogued empirical operation with declared dimension behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmpiricalOp {
    pub name: &'static str,
    pub input_dim: Dim,
    pub output_dim: Dim,
    pub tower_effect: TowerLevel,
    pub description: &'static str,
}

pub use catalog::catalog;
pub use lookup::lookup;
pub use sqrt_psi::sqrt_psi;
