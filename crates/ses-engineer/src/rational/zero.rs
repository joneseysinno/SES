use super::Rational;

impl Rational {
    /// Zero.
    pub const fn zero() -> Self {
        Self { num: 0, den: 1 }
    }
}
