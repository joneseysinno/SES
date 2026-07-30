use crate::repr_error::DimError;

use super::{Dim, add_exp};

impl Dim {
    /// Exponent addition with overflow checking.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, rhs: Self) -> Result<Self, DimError> {
        Ok(Self {
            force: add_exp(self.force, rhs.force, "F")?,
            length: add_exp(self.length, rhs.length, "L")?,
            time: add_exp(self.time, rhs.time, "T")?,
            temp: add_exp(self.temp, rhs.temp, "Θ")?,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn mul_length_to_area() {
        let l = Dim::LENGTH;
        let l2 = l.mul(l).unwrap();
        assert_eq!(l2, Dim::AREA);
    }

    #[test]
    fn stress_from_force_over_area() {
        let stress = Dim::FORCE.div(Dim::AREA).unwrap();
        assert_eq!(stress, Dim::STRESS);
    }
}
