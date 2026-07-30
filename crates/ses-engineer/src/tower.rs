//! Tower arithmetic over adele-ring (Phase 5).

mod lift;
mod narrow;
mod newtypes;

pub use lift::lift;
pub use narrow::narrow;
pub use newtypes::{Area, Force, Length, LineLoad, Moment, Ratio, Stress};

/// Exact rational/algebraic carrier lifted from ℚ (Vocabulary §1.3).
#[derive(Clone)]
pub struct TowerValue(pub(crate) adele_ring::TowerValue);

impl core::fmt::Debug for TowerValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TowerValue(level={:?})", self.0.level())
    }
}

impl TowerValue {
    /// Borrow the inner adele-ring carrier.
    pub fn inner(&self) -> &adele_ring::TowerValue {
        &self.0
    }

    /// Exact addition in the tower.
    pub fn add(&self, other: &Self) -> Self {
        Self(self.0.add(&other.0))
    }

    /// Exact multiplication in the tower.
    pub fn mul(&self, other: &Self) -> Self {
        Self(self.0.mul(&other.0))
    }

    /// Square root; promotes to algebraic when needed.
    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }
}

impl PartialEq for TowerValue {
    fn eq(&self, other: &Self) -> bool {
        match crate::compare::certified_cmp(self.clone(), other.clone()) {
            Ok(decided) => decided.value == core::cmp::Ordering::Equal,
            Err(_) => false,
        }
    }
}
