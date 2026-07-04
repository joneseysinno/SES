//! Shared opaque identifiers (ses-core-build-plan §6).

/// Opaque key into the units space (Vocabulary §2, space 10).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct UnitId(pub u32);
