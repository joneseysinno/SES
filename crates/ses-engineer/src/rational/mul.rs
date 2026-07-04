use crate::repr_error::RationalError;

use super::Rational;

impl Rational {
    /// Exact multiplication.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Self) -> Result<Self, RationalError> {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}
