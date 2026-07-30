use super::Revision;

/// A reference to a persisted record at a specific revision (Vocabulary §1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pin<Id> {
    /// Stable identifier of the pinned record.
    pub id: Id,
    /// Revision observed when the pin was recorded.
    pub at: Revision,
}

impl<Id> Pin<Id> {
    /// Constructs a pin at the given revision.
    pub const fn new(id: Id, at: Revision) -> Self {
        Self { id, at }
    }
}
