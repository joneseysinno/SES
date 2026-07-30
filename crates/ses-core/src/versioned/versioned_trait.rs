use super::lineage::Via;

/// Schema-generation marker for versioned persisted records (Vocabulary §1.2).
///
/// Each generation names its immediate predecessor via [`Supersedes`]. Genesis
/// types use [`Root`](super::Root) as their predecessor and
/// [`Genesis`](super::lineage::Genesis) as [`LineageVia`].
pub trait Versioned: Sized {
    /// Wire-format schema version byte for this generation.
    const VERSION: u8;
    /// Immediate predecessor type in the lineage chain.
    type Supersedes;
    /// Witness for [`Lineage`](super::Lineage) membership.
    type LineageVia: Via<Self>;
}
