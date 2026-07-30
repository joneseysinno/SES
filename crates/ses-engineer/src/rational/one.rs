use super::Rational;

impl Rational {
    /// One.
    pub const fn one() -> Self {
        Self { num: 1, den: 1 }
    }
}
