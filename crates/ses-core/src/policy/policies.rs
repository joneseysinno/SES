use super::Policy;
use super::no_floats::NoFloats;
use super::no_silent_defaults::NoSilentDefaults;
use super::reject_unknown_kinds::RejectUnknownKinds;
use super::single_rounding_event::SingleRoundingEvent;

/// Descriptor for a registered constitutional policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyEntry {
    /// Stable policy name.
    pub name: &'static str,
    /// Policy revision.
    pub version: u32,
    /// Normative statement.
    pub statement: &'static str,
}

impl PolicyEntry {
    const fn of<P: Policy>() -> Self {
        Self {
            name: P::NAME,
            version: P::VERSION,
            statement: P::STATEMENT,
        }
    }
}

/// Inventory of all constitutional policies (ses-core-build-plan §5).
pub fn policies() -> &'static [PolicyEntry] {
    const ENTRIES: &[PolicyEntry] = &[
        PolicyEntry::of::<NoFloats>(),
        PolicyEntry::of::<RejectUnknownKinds>(),
        PolicyEntry::of::<NoSilentDefaults>(),
        PolicyEntry::of::<SingleRoundingEvent>(),
    ];
    ENTRIES
}
