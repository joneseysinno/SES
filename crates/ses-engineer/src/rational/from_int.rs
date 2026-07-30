use super::Rational;

impl Rational {
    /// Integer rational `n/1`.
    pub fn from_int(n: i64) -> Self {
        Self { num: n, den: 1 }
    }
}
