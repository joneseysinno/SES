//! Constitutional policies enforced workspace-wide.

mod no_floats;
mod no_silent_defaults;
mod policies;
mod policy_trait;
mod reject_unknown_kinds;
mod single_rounding_event;

pub use no_floats::NoFloats;
pub use no_silent_defaults::NoSilentDefaults;
pub use policies::{PolicyEntry, policies};
pub use policy_trait::Policy;
pub use reject_unknown_kinds::RejectUnknownKinds;
pub use single_rounding_event::SingleRoundingEvent;
