use crate::error::EngineerError;
use crate::quantity::{Quantity, Rational};
use crate::units::{DimensionSignature, UnitId, UnitSystem};

/// Registry entry for a single unit row (mirrors `Unit` in ses-adapter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitEntry {
    pub id: UnitId,
    pub symbol: String,
    pub name: String,
    pub dim: DimensionSignature,
    /// Exact ratio relative to the dimension pivot unit (registry-internal).
    pub ratio_to_pivot: Rational,
    pub system: UnitSystem,
}

/// In-memory unit registry. Populated from the `units` InfiniteDB space at runtime.
#[derive(Debug, Clone, Default)]
pub struct UnitRegistry {
    entries: Vec<UnitEntry>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn register(&mut self, entry: UnitEntry) {
        self.entries.push(entry);
    }

    pub fn get(&self, id: UnitId) -> Option<&UnitEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn entries(&self) -> &[UnitEntry] {
        &self.entries
    }
}

/// Convert a quantity to a target unit using exact pivot ratios.
/// Full adele-ring-backed implementation deferred to a later phase.
pub fn convert_quantity(
    q: &Quantity,
    target: UnitId,
    registry: &UnitRegistry,
) -> Result<Quantity, EngineerError> {
    if q.unit == target {
        return Ok(q.clone());
    }

    let from = registry
        .get(q.unit)
        .ok_or(EngineerError::UnknownUnit(q.unit))?;
    let to = registry
        .get(target)
        .ok_or(EngineerError::UnknownUnit(target))?;

    if from.dim != to.dim {
        return Err(EngineerError::IncompatibleUnits {
            from: q.unit,
            to: target,
        });
    }

    // Skeleton: identity passthrough with updated unit id; real conversion uses pivot ratios.
    Ok(Quantity {
        value: q.value,
        unit: target,
        authored: q.authored.clone(),
    })
}
