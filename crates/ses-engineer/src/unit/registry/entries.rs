use super::UnitRegistry;

impl UnitRegistry {
    /// Iterate registered entries (Vocabulary §1.3).
    pub fn entries(&self) -> &[super::UnitEntry] {
        &self.entries
    }
}
