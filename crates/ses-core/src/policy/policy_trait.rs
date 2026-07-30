/// Constitutional policy marker (ses-core-build-plan §5).
pub trait Policy {
    /// Stable policy identifier (dot-separated, lowercase).
    const NAME: &'static str;
    /// Policy revision; increment when the statement changes.
    const VERSION: u32;
    /// Human-readable normative statement.
    const STATEMENT: &'static str;
}
