use super::UnitRegistry;

impl UnitRegistry {
    /// Create an empty registry (Vocabulary §1.3).
    #[allow(clippy::new_without_default)] // Ephemeral types forbid Default (NoSilentDefaults).
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}
