use crate::repr_error::DimError;

use super::{Dim, sub_exp};

impl Dim {
    /// Exponent subtraction with overflow checking.
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, rhs: Self) -> Result<Self, DimError> {
        Ok(Self {
            force: sub_exp(self.force, rhs.force, "F")?,
            length: sub_exp(self.length, rhs.length, "L")?,
            time: sub_exp(self.time, rhs.time, "T")?,
            temp: sub_exp(self.temp, rhs.temp, "Θ")?,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn div_inverse_mul() {
        let l = Dim::LENGTH;
        let l2 = l.mul(l).unwrap();
        assert_eq!(l2.div(l).unwrap(), l);
    }
}
