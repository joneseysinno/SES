use crate::repr_error::RationalError;

use super::{Rational, parse_i64, pow10_i64, reduce};

impl Rational {
    /// Parse an exact decimal string without floats, e.g. `"0.0025"` → 1/400.
    pub fn from_decimal_str(s: &str) -> Result<Self, RationalError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(RationalError::ParseError {
                input: s.to_string(),
                reason: "empty",
            });
        }
        if s.starts_with('.') || s.ends_with('.') {
            return Err(RationalError::ParseError {
                input: s.to_string(),
                reason: "bare decimal point not allowed",
            });
        }
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 2 {
            return Err(RationalError::ParseError {
                input: s.to_string(),
                reason: "multiple decimal points",
            });
        }
        if parts.len() == 1 {
            let n = parse_i64(parts[0])?;
            return Ok(Self::from_int(n));
        }
        let int_part = if parts[0].is_empty() {
            0i64
        } else {
            parse_i64(parts[0])?
        };
        let frac_part = parts[1];
        if frac_part.len() > 18 {
            return Err(RationalError::Overflow);
        }
        let frac_digits = frac_part.len();
        let frac_num = parse_i64(frac_part)?;
        let scale = pow10_i64(frac_digits as u32)?;
        let num = int_part
            .checked_mul(scale)
            .and_then(|v| v.checked_add(frac_num))
            .ok_or(RationalError::Overflow)?;
        let den = scale;
        reduce(num, den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_parse() {
        let r = Rational::from_decimal_str("0.0025").unwrap();
        assert_eq!(r, Rational::new(1, 400).unwrap());
        let r = Rational::from_decimal_str("3.0").unwrap();
        assert_eq!(r, Rational::from_int(3));
    }

    #[test]
    fn reject_bare_dot() {
        assert!(Rational::from_decimal_str(".5").is_err());
        assert!(Rational::from_decimal_str("5.").is_err());
    }
}
