use super::Policy;

/// Testimony-bearing types must not derive or implement `Default`.
pub enum NoSilentDefaults {}

impl Policy for NoSilentDefaults {
    const NAME: &'static str = "ses-core.no-silent-defaults";
    const VERSION: u32 = 1;
    const STATEMENT: &'static str = "Types implementing Testimony must be constructed with real witness data; Default is forbidden.";
}
