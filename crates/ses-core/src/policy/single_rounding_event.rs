use super::Policy;

/// Serialization to display/report formats performs at most one rounding event.
pub enum SingleRoundingEvent {}

impl Policy for SingleRoundingEvent {
    const NAME: &'static str = "ses-core.single-rounding-event";
    const VERSION: u32 = 1;
    const STATEMENT: &'static str = "Values pass through tower arithmetic exactly; rounding occurs once at the serialization boundary.";
}
