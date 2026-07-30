use crate::unit::UnitId;

use super::UnitRegistry;

impl UnitRegistry {
    /// Look up a unit entry by id (Vocabulary §1.3).
    pub fn get(&self, id: UnitId) -> Option<&super::UnitEntry> {
        self.entries.iter().find(|e| e.id == id)
    }
}
