use crate::repr_error::{AuthoredErrorKind, AuthoredParseError};

use super::detail::err;
use super::parse_number::{parse_mixed_number, parse_number};
use crate::authored::{Authored, UnitSym};

pub(super) fn parse_feet_inches(
    input: &str,
    base_offset: usize,
) -> Result<Option<Authored>, AuthoredParseError> {
    let Some(pos) = input.find('\'') else {
        return Ok(None);
    };
    let feet_str = input[..pos].trim();
    if feet_str.is_empty() {
        return Err(err(base_offset, AuthoredErrorKind::BadNumber));
    }
    let feet = parse_number(feet_str, base_offset)?;
    let rest = input[pos + 1..].trim_start();
    if rest.is_empty() {
        return Ok(Some(Authored::FeetInches { feet, inches: None }));
    }
    if rest == "-" {
        return Err(err(
            base_offset + pos + 1,
            AuthoredErrorKind::TrailingSeparator,
        ));
    }
    let rest = rest.strip_prefix('-').unwrap_or(rest).trim_start();
    if rest.is_empty() {
        return Err(err(
            base_offset + pos + 1,
            AuthoredErrorKind::TrailingSeparator,
        ));
    }
    let (inches_str, _) = if let Some(end) = rest.find('"') {
        (&rest[..end], &rest[end + 1..])
    } else if let Some(end) = rest.find('″') {
        (&rest[..end], &rest[end + '″'.len_utf8()..])
    } else {
        return Err(err(
            base_offset + pos + 1,
            AuthoredErrorKind::UnexpectedChar,
        ));
    };
    let inches = parse_mixed_number(inches_str, base_offset + pos)?;
    Ok(Some(Authored::FeetInches {
        feet,
        inches: Some(inches),
    }))
}

pub(super) fn parse_quantity(
    input: &str,
    base_offset: usize,
) -> Result<Authored, AuthoredParseError> {
    let mut split_at = None;
    for (i, ch) in input.char_indices() {
        if ch.is_ascii_alphabetic() || ch == '"' || ch == '″' {
            split_at = Some(i);
            break;
        }
    }
    if let Some(i) = split_at {
        if i == 0 {
            return Err(err(base_offset, AuthoredErrorKind::UnitBeforeQuantity));
        }
        let num_str = input[..i].trim_end();
        let unit_str = input[i..].trim();
        let value = parse_number(num_str, base_offset)?;
        let unit = Some(UnitSym(unit_str.to_string()));
        return Ok(Authored::Quantity { value, unit });
    }
    let value = parse_number(input, base_offset)?;
    Ok(Authored::Quantity { value, unit: None })
}
