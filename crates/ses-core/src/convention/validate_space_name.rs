use super::error::ConventionError;

/// Validates a space registration name (ses-vocabulary §4).
///
/// Names are snake_case identifiers, e.g. `"check_results"`.
pub fn validate_space_name(name: &str) -> Result<(), ConventionError> {
    if name.is_empty() {
        return Err(ConventionError::InvalidSpaceName {
            name: name.into(),
            reason: "empty name",
        });
    }
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => {
            return Err(ConventionError::InvalidSpaceName {
                name: name.into(),
                reason: "must start with a lowercase letter",
            });
        }
    }
    if chars.any(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')) {
        return Err(ConventionError::InvalidSpaceName {
            name: name.into(),
            reason: "must be snake_case",
        });
    }
    Ok(())
}
