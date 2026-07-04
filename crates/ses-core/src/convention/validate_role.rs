use super::error::ConventionError;

/// Validates an endpoint role label (ses-vocabulary §5).
///
/// Roles are lowercase identifiers, e.g. `"owner"`, `"component"`.
pub fn validate_role(label: &str) -> Result<(), ConventionError> {
    if label.is_empty() {
        return Err(ConventionError::InvalidRole {
            label: label.into(),
            reason: "empty label",
        });
    }
    let mut chars = label.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => {
            return Err(ConventionError::InvalidRole {
                label: label.into(),
                reason: "must start with a lowercase letter",
            });
        }
    }
    if chars.any(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')) {
        return Err(ConventionError::InvalidRole {
            label: label.into(),
            reason: "contains invalid characters",
        });
    }
    Ok(())
}
