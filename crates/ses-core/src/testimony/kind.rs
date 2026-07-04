/// Epistemic classification of persisted and derived values (Vocabulary §1.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestimonyKind {
    /// Engineer- or author-entered fact; verbatim origin must be retained.
    Authored,
    /// Engine-emitted record from an analysis or check run.
    Emitted,
    /// Imported reference catalog entry (codes, provisions).
    Imported,
    /// Derived at point of use; never persisted as testimony (see [`Ephemeral`](super::Ephemeral)).
    Derived,
}
