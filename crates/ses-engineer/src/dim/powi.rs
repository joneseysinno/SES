use crate::repr_error::DimError;

use super::{Dim, mul_exp};

impl Dim {
    /// Raise each exponent to an integer power.
    pub fn powi(self, n: i8) -> Result<Self, DimError> {
        Ok(Self {
            force: mul_exp(self.force, n, "F")?,
            length: mul_exp(self.length, n, "L")?,
            time: mul_exp(self.time, n, "T")?,
            temp: mul_exp(self.temp, n, "Θ")?,
        })
    }
}
