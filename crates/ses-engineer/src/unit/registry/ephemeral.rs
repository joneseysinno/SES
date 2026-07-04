use ses_core::Ephemeral;

/// Witness labels for registry-backed unit resolution (Vocabulary §1.3).
pub const DERIVED_FROM_REGISTRY: &[&str] = &["units-space"];

impl Ephemeral for super::UnitRegistry {
    const DERIVED_FROM: &'static [&'static str] = DERIVED_FROM_REGISTRY;
}
