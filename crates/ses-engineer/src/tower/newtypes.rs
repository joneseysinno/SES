//! Dimension-typed tower aliases (Vocabulary §1.3).

use crate::dimtype::{Qty, markers};

use super::TowerValue;

/// Length in tower arithmetic.
pub type Length = Qty<markers::L1, TowerValue>;
/// Area in tower arithmetic.
pub type Area = Qty<markers::L2, TowerValue>;
/// Force in tower arithmetic.
pub type Force = Qty<markers::F1, TowerValue>;
/// Moment in tower arithmetic.
pub type Moment = Qty<markers::FL, TowerValue>;
/// Stress / pressure in tower arithmetic.
pub type Stress = Qty<markers::Stress, TowerValue>;
/// Line load in tower arithmetic.
pub type LineLoad = Qty<markers::FperL, TowerValue>;
/// Dimensionless ratio in tower arithmetic.
pub type Ratio = Qty<markers::NoDim, TowerValue>;
