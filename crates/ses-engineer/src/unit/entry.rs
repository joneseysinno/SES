use crate::dim::Dim;
use crate::rational::Rational;

use super::{UnitId, UnitSystem};

/// Registry entry for a single unit row (mirrors `Unit` in ses-adapter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitEntry {
    pub id: UnitId,
    pub symbol: String,
    pub name: String,
    pub dim: Dim,
    /// Exact ratio relative to the dimension pivot unit (registry-internal).
    pub ratio_to_pivot: Rational,
    pub system: UnitSystem,
}
