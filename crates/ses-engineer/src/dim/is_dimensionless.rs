use super::Dim;

impl Dim {
    /// True when all exponents are zero.
    pub fn is_dimensionless(self) -> bool {
        self == Self::DIMENSIONLESS
    }
}
