use crate::repr_error::RationalError;

use super::Rational;

impl Rational {
    /// Exact addition.
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Result<Self, RationalError> {
        Rational::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}
