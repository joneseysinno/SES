//! Runtime dimension algebra over the F·L·T·Θ basis (Vocabulary §1.3).

use core::fmt;

use crate::error::DimError;

/// A physical dimension as an exponent vector over the F·L·T·Θ basis.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Dim {
    /// Force exponent.
    pub force: i8,
    /// Length exponent.
    pub length: i8,
    /// Time exponent.
    pub time: i8,
    /// Temperature exponent.
    pub temp: i8,
}

impl Dim {
    /// Dimensionless quantity (ratios, ρ, φ, αc).
    pub const DIMENSIONLESS: Self = Self::new(0, 0, 0, 0);
    /// Force (F).
    pub const FORCE: Self = Self::new(1, 0, 0, 0);
    /// Length (L).
    pub const LENGTH: Self = Self::new(0, 1, 0, 0);
    /// Time (T).
    pub const TIME: Self = Self::new(0, 0, 1, 0);
    /// Temperature interval (Θ).
    pub const TEMP: Self = Self::new(0, 0, 0, 1);
    /// Stress / pressure (F·L⁻²), e.g. psi, ksi.
    pub const STRESS: Self = Self::new(1, -2, 0, 0);
    /// Moment (F·L), e.g. kip·ft.
    pub const MOMENT: Self = Self::new(1, 1, 0, 0);
    /// Line load (F·L⁻¹).
    pub const FORCE_PER_LENGTH: Self = Self::new(1, -1, 0, 0);
    /// Area (L²).
    pub const AREA: Self = Self::new(0, 2, 0, 0);
    /// Volume (L³).
    pub const VOLUME: Self = Self::new(0, 3, 0, 0);
    /// Unit weight (F·L⁻³).
    pub const FORCE_PER_VOLUME: Self = Self::new(1, -3, 0, 0);
    /// Curvature (L⁻¹).
    pub const INVERSE_LENGTH: Self = Self::new(0, -1, 0, 0);

    /// Construct a dimension from exponent components.
    pub const fn new(force: i8, length: i8, time: i8, temp: i8) -> Self {
        Self {
            force,
            length,
            time,
            temp,
        }
    }

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

    /// Raise each exponent to an integer power.
    pub fn powi(self, n: i8) -> Result<Self, DimError> {
        Ok(Self {
            force: mul_exp(self.force, n, "F")?,
            length: mul_exp(self.length, n, "L")?,
            time: mul_exp(self.time, n, "T")?,
            temp: mul_exp(self.temp, n, "Θ")?,
        })
    }

    /// Invert all exponents (dimension reciprocal).
    pub fn recip(self) -> Result<Self, DimError> {
        self.powi(-1)
    }

    /// True when all exponents are zero.
    pub fn is_dimensionless(self) -> bool {
        self == Self::DIMENSIONLESS
    }
}

fn add_exp(lhs: i8, rhs: i8, axis: &'static str) -> Result<i8, DimError> {
    lhs.checked_add(rhs)
        .ok_or(DimError::ExponentOverflow { axis, lhs, rhs })
}

fn sub_exp(lhs: i8, rhs: i8, axis: &'static str) -> Result<i8, DimError> {
    lhs.checked_sub(rhs)
        .ok_or(DimError::ExponentOverflow { axis, lhs, rhs })
}

fn mul_exp(lhs: i8, rhs: i8, axis: &'static str) -> Result<i8, DimError> {
    lhs.checked_mul(rhs)
        .ok_or(DimError::ExponentOverflow { axis, lhs, rhs })
}

impl Default for Dim {
    fn default() -> Self {
        Self::DIMENSIONLESS
    }
}

impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dimensionless() {
            return f.write_str("—");
        }
        let mut first = true;
        let parts: [(&str, i8); 4] = [
            ("F", self.force),
            ("L", self.length),
            ("T", self.time),
            ("Θ", self.temp),
        ];
        for (sym, exp) in parts {
            if exp == 0 {
                continue;
            }
            if !first {
                f.write_str("·")?;
            }
            first = false;
            f.write_str(sym)?;
            write_exp(f, exp)?;
        }
        Ok(())
    }
}

fn write_exp(f: &mut fmt::Formatter<'_>, exp: i8) -> fmt::Result {
    match exp {
        1 => Ok(()),
        -1 => f.write_str("⁻¹"),
        2 => f.write_str("²"),
        -2 => f.write_str("⁻²"),
        3 => f.write_str("³"),
        -3 => f.write_str("⁻³"),
        4 => f.write_str("⁴"),
        -4 => f.write_str("⁻⁴"),
        n if n > 0 => write!(f, "^{n}"),
        n => write!(f, "^{n}"),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn display_named_constants() {
        assert_eq!(Dim::STRESS.to_string(), "F·L⁻²");
        assert_eq!(Dim::MOMENT.to_string(), "F·L");
        assert_eq!(Dim::DIMENSIONLESS.to_string(), "—");
    }

    #[test]
    fn mul_div_inverse() {
        let l = Dim::LENGTH;
        let l2 = l.mul(l).unwrap();
        assert_eq!(l2, Dim::AREA);
        assert_eq!(l2.div(l).unwrap(), l);
        assert_eq!(l.recip().unwrap(), Dim::INVERSE_LENGTH);
    }

    #[test]
    fn stress_from_force_over_area() {
        let stress = Dim::FORCE.div(Dim::AREA).unwrap();
        assert_eq!(stress, Dim::STRESS);
    }
}

#[cfg(test)]
mod proptests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mul_associative(a in -3i8..=3, b in -3i8..=3, c in -3i8..=3) {
            let da = Dim::new(a, 0, 0, 0);
            let db = Dim::new(b, 0, 0, 0);
            let dc = Dim::new(c, 0, 0, 0);
            if let (Ok(ab), Ok(bc)) = (da.mul(db), db.mul(dc)) {
                if let (Ok(ab_c), Ok(a_bc)) = (ab.mul(dc), da.mul(bc)) {
                    prop_assert_eq!(ab_c, a_bc);
                }
            }
        }

        #[test]
        fn identity_mul(a in -3i8..=3) {
            let d = Dim::new(a, a, a, a);
            if let Ok(dm) = d.mul(Dim::DIMENSIONLESS) {
                prop_assert_eq!(dm, d);
            }
        }
    }
}
