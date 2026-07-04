use crate::repr_error::{AuthoredErrorKind, AuthoredParseError};

use super::detail::{contains_spaced_x, err};
use crate::authored::Authored;

pub(super) fn parse_product(input: &str) -> Result<Authored, AuthoredParseError> {
    let parts: Vec<&str> = if input.contains('×') {
        input.split('×').map(str::trim).collect()
    } else {
        split_spaced_x(input)?
    };
    if parts.len() < 2 {
        return Err(err(0, AuthoredErrorKind::AdjacentMeasures));
    }
    let mut items = Vec::with_capacity(parts.len());
    for part in parts {
        if part.is_empty() {
            return Err(err(0, AuthoredErrorKind::AdjacentMeasures));
        }
        items.push(parse_single(part, 0)?);
    }
    Ok(Authored::Product(items))
}

fn split_spaced_x(input: &str) -> Result<Vec<&str>, AuthoredParseError> {
    let mut parts = Vec::new();
    let bytes = input.as_bytes();
    let mut start = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        let is_x = bytes[i] == b'x' || bytes[i] == b'X';
        let spaced = is_x
            && i > 0
            && i + 1 < bytes.len()
            && bytes[i - 1].is_ascii_whitespace()
            && bytes[i + 1].is_ascii_whitespace();
        if spaced {
            if start < i {
                let end = i - 1;
                if start <= end {
                    parts.push(input[start..end].trim());
                }
            }
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            start = i;
            continue;
        }
        i += 1;
    }
    if start <= input.len() {
        parts.push(input[start..].trim());
    }
    Ok(parts)
}

pub(super) fn parse_single(
    input: &str,
    base_offset: usize,
) -> Result<Authored, AuthoredParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(err(base_offset, AuthoredErrorKind::EmptyInput));
    }
    if input.contains('x') && !input.contains('×') && !contains_spaced_x(input) {
        return Err(err(base_offset, AuthoredErrorKind::AmbiguousProduct));
    }
    if let Some(fi) = super::parse_feet_inches::parse_feet_inches(input, base_offset)? {
        return Ok(fi);
    }
    super::parse_measure::parse_quantity(input, base_offset)
}
