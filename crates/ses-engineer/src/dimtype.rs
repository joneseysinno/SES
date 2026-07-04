//! Type-level closed dimension algebra (ses-core-build-plan §4).

#![allow(clippy::arithmetic_side_effects)]

use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Sub};

use crate::dim::Dim;

mod sealed {
    pub trait Sealed {}
}

/// Marker tying a type-level dimension to its runtime [`Dim`].
pub trait DimType: sealed::Sealed {
    /// Runtime dimension value.
    const DIM: Dim;
    /// Short name for errors and debugging.
    const NAME: &'static str;
}

/// Scalar tagged with a compile-time dimension.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Qty<D: DimType, S> {
    value: S,
    _dim: PhantomData<D>,
}

impl<D: DimType, S> Qty<D, S> {
    /// Wrap a scalar with dimension `D`.
    pub fn new(value: S) -> Self {
        Self {
            value,
            _dim: PhantomData,
        }
    }

    /// Unwrap the inner scalar.
    pub fn into_inner(self) -> S {
        self.value
    }

    /// Borrow the inner scalar.
    pub fn get(&self) -> &S {
        &self.value
    }

    /// Map the scalar while preserving the dimension tag.
    pub fn map<T>(self, f: impl FnOnce(S) -> T) -> Qty<D, T> {
        Qty::new(f(self.value))
    }
}

/// Type-level multiplication result.
pub trait DimMul<Rhs: DimType>: DimType {
    /// Output dimension marker.
    type Out: DimType;
}

/// Type-level division result.
pub trait DimDiv<Rhs: DimType>: DimType {
    /// Output dimension marker.
    type Out: DimType;
}

impl<Da, Db, S> Mul<Qty<Db, S>> for Qty<Da, S>
where
    Da: DimMul<Db>,
    S: Mul<Output = S>,
    Db: DimType,
{
    type Output = Qty<<Da as DimMul<Db>>::Out, S>;

    fn mul(self, rhs: Qty<Db, S>) -> Self::Output {
        Qty::new(self.value * rhs.value)
    }
}

impl<Da, Db, S> Div<Qty<Db, S>> for Qty<Da, S>
where
    Da: DimDiv<Db>,
    S: Div<Output = S>,
    Db: DimType,
{
    type Output = Qty<<Da as DimDiv<Db>>::Out, S>;

    fn div(self, rhs: Qty<Db, S>) -> Self::Output {
        Qty::new(self.value / rhs.value)
    }
}

impl<D, S> Add<Qty<D, S>> for Qty<D, S>
where
    D: DimType,
    S: Add<Output = S>,
{
    type Output = Qty<D, S>;

    fn add(self, rhs: Qty<D, S>) -> Self::Output {
        Qty::new(self.value + rhs.value)
    }
}

impl<D, S> Sub<Qty<D, S>> for Qty<D, S>
where
    D: DimType,
    S: Sub<Output = S>,
{
    type Output = Qty<D, S>;

    fn sub(self, rhs: Qty<D, S>) -> Self::Output {
        Qty::new(self.value - rhs.value)
    }
}

macro_rules! dim_marker {
    ($name:ident, $dim:expr, $label:literal) => {
        #[doc = $label]
        pub struct $name;
        impl sealed::Sealed for $name {}
        impl DimType for $name {
            const DIM: Dim = $dim;
            const NAME: &'static str = stringify!($name);
        }
    };
}

dim_marker!(NoDim, Dim::DIMENSIONLESS, "Dimensionless ratios and factors");
dim_marker!(L1, Dim::LENGTH, "Length (L)");
dim_marker!(L2, Dim::AREA, "Area (L²)");
dim_marker!(L3, Dim::VOLUME, "Volume (L³)");
dim_marker!(L4, Dim::new(0, 4, 0, 0), "Second moment of area (L⁴)");
dim_marker!(InvL, Dim::INVERSE_LENGTH, "Curvature (L⁻¹)");
dim_marker!(F1, Dim::FORCE, "Force (F)");
dim_marker!(FL, Dim::MOMENT, "Moment (F·L)");
dim_marker!(FperL, Dim::FORCE_PER_LENGTH, "Line load (F·L⁻¹)");
dim_marker!(Stress, Dim::STRESS, "Stress / pressure (F·L⁻²)");
dim_marker!(FperL3, Dim::FORCE_PER_VOLUME, "Unit weight (F·L⁻³)");
dim_marker!(T1, Dim::TIME, "Time (T)");
dim_marker!(Theta, Dim::TEMP, "Temperature interval (Θ)");

macro_rules! dim_mul {
    ($a:ident, $b:ident, $out:ident) => {
        impl DimMul<$b> for $a {
            type Out = $out;
        }
    };
}

dim_mul!(L1, L1, L2);
dim_mul!(L2, L1, L3);
dim_mul!(L3, L1, L4);
dim_mul!(F1, L1, FL);
dim_mul!(L1, F1, FL);
dim_mul!(Stress, L2, F1);
dim_mul!(L2, Stress, F1);
dim_mul!(Stress, L1, FperL);
dim_mul!(L1, Stress, FperL);

macro_rules! dim_div {
    ($a:ident / $b:ident => $out:ident) => {
        impl DimDiv<$b> for $a {
            type Out = $out;
        }
    };
}

dim_div!(F1 / L1 => FperL);
dim_div!(F1 / L2 => Stress);
dim_div!(F1 / L3 => FperL3);
dim_div!(FL / L1 => F1);
dim_div!(FL / F1 => L1);

macro_rules! dim_identity_mul {
    ($x:ident) => {
        impl DimMul<NoDim> for $x {
            type Out = $x;
        }
        impl DimMul<$x> for NoDim {
            type Out = $x;
        }
    };
}

macro_rules! dim_identity_div {
    ($x:ident) => {
        impl DimDiv<$x> for $x {
            type Out = NoDim;
        }
    };
}

dim_identity_mul!(L1);
dim_identity_mul!(L2);
dim_identity_mul!(L3);
dim_identity_mul!(L4);
dim_identity_mul!(InvL);
dim_identity_mul!(F1);
dim_identity_mul!(FL);
dim_identity_mul!(FperL);
dim_identity_mul!(Stress);
dim_identity_mul!(FperL3);
dim_identity_mul!(T1);
dim_identity_mul!(Theta);

dim_identity_div!(L1);
dim_identity_div!(L2);
dim_identity_div!(F1);
dim_identity_div!(Stress);
dim_identity_div!(FL);

#[cfg(test)]
mod consistency {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn mul_l1_l1_l2() {
        assert_eq!(
            <L1 as DimType>::DIM.mul(<L1 as DimType>::DIM).unwrap(),
            <L2 as DimType>::DIM
        );
    }

    #[test]
    fn mul_l2_l1_l3() {
        assert_eq!(
            <L2 as DimType>::DIM.mul(<L1 as DimType>::DIM).unwrap(),
            <L3 as DimType>::DIM
        );
    }

    #[test]
    fn mul_f1_l1_fl() {
        assert_eq!(
            <F1 as DimType>::DIM.mul(<L1 as DimType>::DIM).unwrap(),
            <FL as DimType>::DIM
        );
    }

    #[test]
    fn div_f1_l2_stress() {
        assert_eq!(
            <F1 as DimType>::DIM.div(<L2 as DimType>::DIM).unwrap(),
            <Stress as DimType>::DIM
        );
    }

    #[test]
    fn qty_length_mul() {
        let a: Qty<L1, i64> = Qty::new(24);
        let b: Qty<L1, i64> = Qty::new(2);
        let area: Qty<L2, i64> = a * b;
        assert_eq!(area.into_inner(), 48);
    }
}
