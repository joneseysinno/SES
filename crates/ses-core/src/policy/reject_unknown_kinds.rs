use super::Policy;

/// Hyperedge and fact kind catalogs reject unknown labels at registration.
pub enum RejectUnknownKinds {}

impl Policy for RejectUnknownKinds {
    const NAME: &'static str = "ses-core.reject-unknown-kinds";
    const VERSION: u32 = 1;
    const STATEMENT: &'static str =
        "Only convention-valid edge kinds, roles, and fact names may be registered.";
}
