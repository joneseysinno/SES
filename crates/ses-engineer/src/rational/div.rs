use crate::repr_error::RationalError;

use super::Rational;

impl Rational {
    /// Exact division.
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, other: Self) -> Result<Self, RationalError> {
        if other.num == 0 {
            return Err(RationalError::ZeroDenominator);
        }
        Rational::new(self.num * other.den, self.den * other.num)
    }
}
