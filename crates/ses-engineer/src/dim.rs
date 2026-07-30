//! Runtime dimension algebra over the F·L·T·Θ basis (Vocabulary §1.3).

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
}

mod default;
mod deserialize;
mod detail;
mod display;
mod div;
mod is_dimensionless;
mod mul;
mod new;
mod powi;
mod recip;
mod serialize;

pub(crate) use detail::{add_exp, mul_exp, sub_exp};
