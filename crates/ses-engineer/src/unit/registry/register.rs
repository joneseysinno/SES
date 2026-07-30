use super::{UnitEntry, UnitRegistry};

impl UnitRegistry {
    /// Register a unit entry (Vocabulary §1.3).
    pub fn register(&mut self, entry: UnitEntry) {
        self.entries.push(entry);
    }
}
