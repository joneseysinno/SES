use crate::repr_error::RationalError;

use super::{Rational, reduce};

impl Rational {
    /// Construct a reduced rational. Sign is normalized to the numerator.
    pub fn new(num: i64, den: i64) -> Result<Self, RationalError> {
        if den == 0 {
            return Err(RationalError::ZeroDenominator);
        }
        reduce(num, den)
    }
}

#[cfg(test)]
mod tests {
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
}
