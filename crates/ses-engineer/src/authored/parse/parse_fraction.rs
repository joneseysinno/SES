use crate::rational::Rational;
use crate::repr_error::{AuthoredErrorKind, AuthoredParseError};

use super::detail::err;

pub(super) fn parse_fraction(s: &str, base_offset: usize) -> Result<Rational, AuthoredParseError> {
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

pub(super) fn add_rationals(
    a: Rational,
    b: Rational,
    base_offset: usize,
) -> Result<Rational, AuthoredParseError> {
    let num = (a.num() as i128) * (b.den() as i128) + (b.num() as i128) * (a.den() as i128);
    let den = (a.den() as i128) * (b.den() as i128);
    if num > i64::MAX as i128 || num < i64::MIN as i128 || den > i64::MAX as i128 {
        return Err(err(base_offset, AuthoredErrorKind::BadNumber));
    }
    Rational::new(num as i64, den as i64)
        .map_err(|_| err(base_offset, AuthoredErrorKind::BadNumber))
}
