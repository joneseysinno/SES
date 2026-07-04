use crate::rational::Rational;
use crate::repr_error::{AuthoredErrorKind, AuthoredParseError};

use super::detail::err;
use super::parse_fraction::{add_rationals, parse_fraction};

pub(super) fn parse_mixed_number(
    s: &str,
    base_offset: usize,
) -> Result<Rational, AuthoredParseError> {
    let s = s.trim();
    if s.contains('/') {
        if s.contains(' ') {
            let mut parts = s.split_whitespace();
            let whole = parts
                .next()
                .ok_or_else(|| err(base_offset, AuthoredErrorKind::BadNumber))?;
            let frac = parts
                .next()
                .ok_or_else(|| err(base_offset, AuthoredErrorKind::BadNumber))?;
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

pub(super) fn parse_number(s: &str, base_offset: usize) -> Result<Rational, AuthoredParseError> {
    let s = s.trim();
    if s.contains(' ') {
        return parse_mixed_number(s, base_offset);
    }
    if s.contains('/') {
        return parse_fraction(s, base_offset);
    }
    Rational::from_decimal_str(s).map_err(|_| err(base_offset, AuthoredErrorKind::BadNumber))
}
