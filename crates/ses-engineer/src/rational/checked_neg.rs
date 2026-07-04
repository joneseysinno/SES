use crate::repr_error::RationalError;

use super::Rational;

impl Rational {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_negation_overflow() {
        let r = Rational::new(i64::MIN, 1).unwrap();
        assert_eq!(r.checked_neg(), Err(RationalError::Overflow));
    }
}
