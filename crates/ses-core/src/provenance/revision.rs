/// Monotonic revision counter for a persisted record (Vocabulary §1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Revision(pub u64);

impl Revision {
    /// Zero revision — genesis write.
    pub const ZERO: Self = Self(0);

    /// Returns the raw counter value.
    pub const fn get(self) -> u64 {
        self.0
    }
}
