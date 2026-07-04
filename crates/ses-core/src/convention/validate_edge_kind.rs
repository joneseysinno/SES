use super::error::ConventionError;

fn is_ident_start(c: char) -> bool {
    c.is_ascii_lowercase()
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'
}

fn validate_segment(segment: &str) -> Result<(), &'static str> {
    let mut chars = segment.chars();
    match chars.next() {
        None => return Err("empty segment"),
        Some(c) if !is_ident_start(c) => return Err("segment must start with a lowercase letter"),
        Some(_) => {}
    }
    if chars.any(|c| !is_ident_continue(c)) {
        return Err("segment contains invalid characters");
    }
    Ok(())
}

/// Validates a hyperedge kind label (ses-vocabulary §5).
///
/// Labels are lowercase dot-separated identifiers, e.g. `"project.contains"`.
pub fn validate_edge_kind(label: &str) -> Result<(), ConventionError> {
    if label.is_empty() {
        return Err(ConventionError::InvalidEdgeKind {
            label: label.into(),
            reason: "empty label",
        });
    }
    let mut segments = label.split('.');
    let first = segments.next().unwrap_or("");
    validate_segment(first).map_err(|reason| ConventionError::InvalidEdgeKind {
        label: label.into(),
        reason,
    })?;
    let second = segments
        .next()
        .ok_or_else(|| ConventionError::InvalidEdgeKind {
            label: label.into(),
            reason: "must contain at least one dot",
        })?;
    validate_segment(second).map_err(|reason| ConventionError::InvalidEdgeKind {
        label: label.into(),
        reason,
    })?;
    for segment in segments {
        validate_segment(segment).map_err(|reason| ConventionError::InvalidEdgeKind {
            label: label.into(),
            reason,
        })?;
    }
    Ok(())
}
