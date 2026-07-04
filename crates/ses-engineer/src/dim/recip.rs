use super::Dim;

impl Dim {
    /// Invert all exponents (dimension reciprocal).
    pub fn recip(self) -> Result<Self, crate::repr_error::DimError> {
        self.powi(-1)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn recip_length() {
        assert_eq!(Dim::LENGTH.recip().unwrap(), Dim::INVERSE_LENGTH);
    }
}
