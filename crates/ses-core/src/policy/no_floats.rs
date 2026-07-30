use super::Policy;

/// No floating-point types in persisted or computed engineering state.
pub enum NoFloats {}

impl Policy for NoFloats {
    const NAME: &'static str = "ses-core.no-floats";
    const VERSION: u32 = 1;
    const STATEMENT: &'static str = "Engineering values use exact rationals or tower arithmetic; IEEE-754 floats are forbidden.";
}
