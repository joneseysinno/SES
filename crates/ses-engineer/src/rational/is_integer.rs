use super::Rational;

impl Rational {
    /// True when the denominator is 1.
    pub fn is_integer(self) -> bool {
        self.den == 1
    }
}
