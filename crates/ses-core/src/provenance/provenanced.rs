use super::{Pin, Revision};

/// Types that expose their revision and upstream input pins (Vocabulary §1.2).
pub trait Provenanced {
    /// Identifier type for [`Pin`] records this value depends on.
    type PinId: Clone + Eq;
    /// Current revision of this record.
    fn revision(&self) -> Revision;
    /// Upstream pins recorded when this value was produced.
    fn pins(&self) -> &[Pin<Self::PinId>];
}
