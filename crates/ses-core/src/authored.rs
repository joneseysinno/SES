//! Authored-text grammar and parser (Vocabulary §1.3).
//!
//! Pure syntax — unit symbols stay unresolved strings until `ses-engineer` evaluates.

#![allow(clippy::arithmetic_side_effects)]

use crate::error::{AuthoredErrorKind, AuthoredParseError};
use crate::rational::Rational;

/// A unit symbol exactly as written, unresolved.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnitSym(pub String);

/// Syntactic AST for engineer-authored quantity text.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Authored {
    /// Simple number with optional unit, e.g. `24 ft`, `0.0025`.
    Quantity {
        /// Parsed numeric value.
        value: Rational,
        /// Unit symbol if present.
        unit: Option<UnitSym>,
    },
    /// Feet-inches form, e.g. `3'-6"`, `3'-6 1/2"`.
    FeetInches {
        /// Whole feet.
        feet: Rational,
        /// Inches component if present.
        inches: Option<Rational>,
    },
    /// Product of measures, e.g. `3 × 8 ft`.
    Product(Vec<Authored>),
}

/// Parse engineer-authored quantity text.
pub fn parse_authored(input: &str) -> Result<Authored, AuthoredParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(err(0, AuthoredErrorKind::EmptyInput));
    }
    if trimmed.contains('×') || contains_spaced_x(trimmed) {
        return parse_product(trimmed);
    }
    parse_single(trimmed, 0)
}

/// Canonical rendering (not guaranteed byte-identical to input).
pub fn render(ast: &Authored) -> String {
    match ast {
        Authored::Quantity { value, unit } => {
            let mut s = value.to_string();
            if let Some(u) = unit {
                s.push(' ');
                s.push_str(&u.0);
            }
            s
        }
        Authored::FeetInches { feet, inches } => {
            let mut s = format!("{feet}'");
            if let Some(inches) = inches {
                s.push('-');
                s.push_str(&inches.to_string());
                s.push('"');
            }
            s
        }
        Authored::Product(items) => items
            .iter()
            .map(render)
            .collect::<Vec<_>>()
            .join(" × "),
    }
}

fn contains_spaced_x(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 1..bytes.len().saturating_sub(1) {
        if (bytes[i] == b'x' || bytes[i] == b'X')
            && bytes[i - 1].is_ascii_whitespace()
            && bytes[i + 1].is_ascii_whitespace()
        {
            return true;
        }
    }
    false
}

fn parse_product(input: &str) -> Result<Authored, AuthoredParseError> {
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

fn parse_single(input: &str, base_offset: usize) -> Result<Authored, AuthoredParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(err(base_offset, AuthoredErrorKind::EmptyInput));
    }
    if input.contains('x') && !input.contains('×') && !contains_spaced_x(input) {
        return Err(err(base_offset, AuthoredErrorKind::AmbiguousProduct));
    }
    if let Some(fi) = parse_feet_inches(input, base_offset)? {
        return Ok(fi);
    }
    parse_quantity(input, base_offset)
}

fn parse_feet_inches(input: &str, base_offset: usize) -> Result<Option<Authored>, AuthoredParseError> {
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
        return Ok(Some(Authored::FeetInches {
            feet,
            inches: None,
        }));
    }
    if rest == "-" {
        return Err(err(base_offset + pos + 1, AuthoredErrorKind::TrailingSeparator));
    }
    let rest = rest.strip_prefix('-').unwrap_or(rest).trim_start();
    if rest.is_empty() {
        return Err(err(base_offset + pos + 1, AuthoredErrorKind::TrailingSeparator));
    }
    let (inches_str, _) = if let Some(end) = rest.find('"') {
        (&rest[..end], &rest[end + 1..])
    } else if let Some(end) = rest.find('″') {
        (&rest[..end], &rest[end + '″'.len_utf8()..])
    } else {
        return Err(err(base_offset + pos + 1, AuthoredErrorKind::UnexpectedChar));
    };
    let inches = parse_mixed_number(inches_str, base_offset + pos)?;
    Ok(Some(Authored::FeetInches {
        feet,
        inches: Some(inches),
    }))
}

fn parse_quantity(input: &str, base_offset: usize) -> Result<Authored, AuthoredParseError> {
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
    Ok(Authored::Quantity {
        value,
        unit: None,
    })
}

fn parse_mixed_number(s: &str, base_offset: usize) -> Result<Rational, AuthoredParseError> {
    let s = s.trim();
    if s.contains('/') {
        if s.contains(' ') {
            let mut parts = s.split_whitespace();
            let whole = parts.next().ok_or_else(|| err(base_offset, AuthoredErrorKind::BadNumber))?;
            let frac = parts.next().ok_or_else(|| err(base_offset, AuthoredErrorKind::BadNumber))?;
            if parts.next().is_some() {
                return Err(err(base_offset, AuthoredErrorKind::BadNumber));
            }
            let w = parse_number(whole, base_offset)?;
            let f = parse_fraction(frac, base_offset)?;
            return add_rationals(w, f, base_offset);
        }
        return parse_fraction(s, base_offset);
    }
    parse_number(s, base_offset)
}

fn parse_number(s: &str, base_offset: usize) -> Result<Rational, AuthoredParseError> {
    let s = s.trim();
    if s.contains(' ') {
        return parse_mixed_number(s, base_offset);
    }
    if s.contains('/') {
        return parse_fraction(s, base_offset);
    }
    Rational::from_decimal_str(s).map_err(|_| err(base_offset, AuthoredErrorKind::BadNumber))
}

fn parse_fraction(s: &str, base_offset: usize) -> Result<Rational, AuthoredParseError> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 2 {
        return Err(err(base_offset, AuthoredErrorKind::BadFraction));
    }
    let num = parts[0]
        .trim()
        .parse::<i64>()
        .map_err(|_| err(base_offset, AuthoredErrorKind::BadFraction))?;
    let den = parts[1]
        .trim()
        .parse::<i64>()
        .map_err(|_| err(base_offset, AuthoredErrorKind::BadFraction))?;
    if den == 0 {
        return Err(err(base_offset, AuthoredErrorKind::BadFraction));
    }
    Rational::new(num, den).map_err(|_| err(base_offset, AuthoredErrorKind::BadFraction))
}

fn add_rationals(a: Rational, b: Rational, base_offset: usize) -> Result<Rational, AuthoredParseError> {
    let num = (a.num() as i128) * (b.den() as i128) + (b.num() as i128) * (a.den() as i128);
    let den = (a.den() as i128) * (b.den() as i128);
    if num > i64::MAX as i128 || num < i64::MIN as i128 || den > i64::MAX as i128 {
        return Err(err(base_offset, AuthoredErrorKind::BadNumber));
    }
    Rational::new(num as i64, den as i64).map_err(|_| err(base_offset, AuthoredErrorKind::BadNumber))
}

fn err(offset: usize, kind: AuthoredErrorKind) -> AuthoredParseError {
    AuthoredParseError { offset, kind }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    fn parse(s: &str) -> Authored {
        parse_authored(s).expect("should parse")
    }

    #[test]
    fn quantity_with_unit() {
        let ast = parse("24 ft");
        assert!(matches!(ast, Authored::Quantity { .. }));
    }

    #[test]
    fn feet_inches() {
        let ast = parse("3'-6\"");
        assert!(matches!(ast, Authored::FeetInches { .. }));
    }

    #[test]
    fn feet_inches_fraction() {
        parse("3'-6 1/2\"");
    }

    #[test]
    fn feet_only() {
        parse("3'");
    }

    #[test]
    fn inches_only() {
        parse("6\"");
    }

    #[test]
    fn mixed_number() {
        parse("5 1/2 in");
    }

    #[test]
    fn product_times() {
        parse("3 × 8 ft");
    }

    #[test]
    fn product_x() {
        parse("2 x 12 in");
    }

    #[test]
    fn decimal_dimensionless() {
        let ast = parse("0.0025");
        if let Authored::Quantity { value, unit } = ast {
            assert_eq!(value, Rational::from_decimal_str("0.0025").unwrap());
            assert!(unit.is_none());
        } else {
            panic!("expected quantity");
        }
    }

    #[test]
    fn stress_unit() {
        parse("5000 psi");
        parse("1.5 kip/ft");
    }

    #[test]
    fn reject_ambiguous() {
        assert!(parse_authored("3x8").is_err());
    }

    #[test]
    fn reject_empty() {
        assert!(parse_authored("").is_err());
    }

    #[test]
    fn reject_unit_before_qty() {
        assert!(parse_authored("ft 24").is_err());
    }

    #[test]
    fn reject_trailing_separator() {
        assert!(parse_authored("3'-").is_err());
    }

    #[test]
    fn reject_zero_fraction() {
        assert!(parse_authored("1/0 in").is_err());
    }

    #[test]
    fn round_trip_idempotent() {
        for s in [
            "24 ft",
            "3'-6\"",
            "0.0025",
            "5000 psi",
            "3 × 8 ft",
        ] {
            let once = parse_authored(s).unwrap();
            let rendered = render(&once);
            let twice = parse_authored(&rendered).unwrap();
            let rerendered = render(&twice);
            assert_eq!(render(&once), rerendered, "round-trip failed for {s}");
        }
    }

    #[test]
    fn never_panics_on_adversarial_inputs() {
        let cases = [
            "",
            "x",
            "xxx",
            "3x8",
            "''''",
            " × ",
            &"x ".repeat(64),
            "3'-6\"'-6\"",
        ];
        for s in cases {
            let _ = parse_authored(s);
        }
    }
}

#[cfg(test)]
mod proptests {}
