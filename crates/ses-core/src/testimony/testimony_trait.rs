use super::TestimonyKind;

/// Marker for types that carry persisted testimony (Vocabulary §1.1).
///
/// Implementors must declare their [`TestimonyKind`] and the witness labels
/// that justify the record's epistemic status.
pub trait Testimony {
    /// Which testimony domain this type belongs to.
    const KIND: TestimonyKind;
    /// Human-readable witness labels (e.g. `"engineer"`, `"analysis-run"`).
    const WITNESSES: &'static [&'static str];
}
