use super::error::ConventionError;

fn all_ascii_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

fn validate_prefixed_two(name: &str, prefix: &str) -> Result<(), ConventionError> {
    let rest = name
        .strip_prefix(prefix)
        .ok_or_else(|| ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "unexpected prefix",
        })?;
    let seq = rest
        .strip_prefix(':')
        .ok_or_else(|| ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "missing colon after prefix",
        })?;
    if !all_ascii_digits(seq) {
        return Err(ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "sequence must be decimal digits",
        });
    }
    Ok(())
}

fn validate_combo_run(name: &str, prefix: &str) -> Result<(), ConventionError> {
    let rest = name
        .strip_prefix(prefix)
        .ok_or_else(|| ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "unexpected prefix",
        })?;
    let body = rest
        .strip_prefix(':')
        .ok_or_else(|| ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "missing first colon",
        })?;
    let (proj, tail) = body
        .split_once(':')
        .ok_or_else(|| ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "missing second colon",
        })?;
    if !all_ascii_digits(proj) || !all_ascii_digits(tail) {
        return Err(ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "both sequences must be decimal digits",
        });
    }
    Ok(())
}

/// Validates a persisted counter name (ses-vocabulary §4).
pub fn validate_counter_name(name: &str) -> Result<(), ConventionError> {
    match name {
        "proj" | "edge" => Ok(()),
        _ if name.starts_with("elem:") => validate_prefixed_two(name, "elem"),
        _ if name.starts_with("matl:") => validate_prefixed_two(name, "matl"),
        _ if name.starts_with("combo:") => validate_combo_run(name, "combo"),
        _ if name.starts_with("run:") => validate_combo_run(name, "run"),
        _ => Err(ConventionError::InvalidCounterName {
            name: name.into(),
            reason: "unknown counter pattern",
        }),
    }
}
