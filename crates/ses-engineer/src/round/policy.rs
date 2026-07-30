use ses_core::policy::Policy;

/// Single rounding event policy (Vocabulary §1.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundingPolicyV1;

impl RoundingPolicyV1 {
    /// Decimal places applied at the serialization boundary.
    pub const DECIMAL_PLACES: u32 = 6;
}

impl Policy for RoundingPolicyV1 {
    const NAME: &'static str = "ses-engineer.single-rounding-event";
    const VERSION: u32 = 1;
    const STATEMENT: &'static str = "Values pass through tower arithmetic exactly; rounding occurs once at the serialization boundary to six decimal places.";
}
