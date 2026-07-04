//! Inert exact rational for storage (Vocabulary §1.3).
//!
//! Representation and ordering only — general arithmetic lives in `ses-engineer`.

#![allow(clippy::arithmetic_side_effects)]

use core::cmp::Ordering;
use core::fmt;

use crate::error::RationalError;

/// Exact rational in reduced form.
///
/// Invariants: `den > 0`, `gcd(|num|, den) == 1`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    /// Construct a reduced rational. Sign is normalized to the numerator.
    pub fn new(num: i64, den: i64) -> Result<Self, RationalError> {
        if den == 0 {
            return Err(RationalError::ZeroDenominator);
        }
        reduce(num, den)
    }

    /// Zero.
    pub const fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    /// One.
    pub const fn one() -> Self {
        Self { num: 1, den: 1 }
    }

    /// Integer rational `n/1`.
    pub fn from_int(n: i64) -> Self {
        Self { num: n, den: 1 }
    }

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

    /// Numerator.
    pub fn num(self) -> i64 {
        self.num
    }

    /// Denominator (always positive).
    pub fn den(self) -> i64 {
        self.den
    }

    /// True when the denominator is 1.
    pub fn is_integer(self) -> bool {
        self.den == 1
    }

    /// Negate with `i64::MIN` guard.
    pub fn checked_neg(self) -> Result<Self, RationalError> {
        if self.num == i64::MIN {
            return Err(RationalError::Overflow);
        }
        Ok(Self {
            num: -self.num,
            den: self.den,
        })
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.num == other.num && self.den == other.den {
            return Ordering::Equal;
        }
        let lhs = (self.num as i128) * (other.den as i128);
        let rhs = (other.num as i128) * (self.den as i128);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Rational {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            return write!(f, "{}", self.num);
        }
        write!(f, "{}/{}", self.num, self.den)
    }
}

fn reduce(num: i64, den: i64) -> Result<Rational, RationalError> {
    if den == 0 {
        return Err(RationalError::ZeroDenominator);
    }
    let (mut num, mut den) = (num, den);
    if den < 0 {
        num = num.checked_neg().ok_or(RationalError::Overflow)?;
        den = den.checked_neg().ok_or(RationalError::Overflow)?;
    }
    let num_abs_i128: i128 = match num.checked_abs() {
        Some(v) => i128::from(v),
        None => 1i128 << 63,
    };
    let g = gcd_i128(num_abs_i128, i128::from(den)) as i64;
    let num_reduced = num / g;
    let den_reduced = den / g;
    Ok(Rational {
        num: num_reduced,
        den: den_reduced,
    })
}

fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn parse_i64(s: &str) -> Result<i64, RationalError> {
    s.parse::<i64>().map_err(|_| RationalError::ParseError {
        input: s.to_string(),
        reason: "invalid integer",
    })
}

fn pow10_i64(exp: u32) -> Result<i64, RationalError> {
    let mut v: i64 = 1;
    for _ in 0..exp {
        v = v.checked_mul(10).ok_or(RationalError::Overflow)?;
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn zero_denominator() {
        assert_eq!(Rational::new(1, 0), Err(RationalError::ZeroDenominator));
    }

    #[test]
    fn sign_normalization() {
        let r = Rational::new(1, -2).unwrap();
        assert_eq!(r.num(), -1);
        assert_eq!(r.den(), 2);
    }

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

    #[test]
    fn min_negation_overflow() {
        let r = Rational::new(i64::MIN, 1).unwrap();
        assert_eq!(r.checked_neg(), Err(RationalError::Overflow));
    }

    #[test]
    fn display_format() {
        assert_eq!(Rational::from_int(3).to_string(), "3");
        assert_eq!(Rational::new(3, 4).unwrap().to_string(), "3/4");
    }

    #[test]
    fn ord_cross_multiply() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(2, 5).unwrap();
        assert!(a < b);
    }
}

#[cfg(test)]
mod proptests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn new_invariants(num in 1i64..10_000, den in 1i64..10_000) {
            let r = Rational::new(num, den).unwrap();
            prop_assert!(r.den() > 0);
            prop_assert_eq!(gcd_i128(r.num().abs() as i128, r.den() as i128), 1);
        }
    }
}
