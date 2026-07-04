//! Type-level closed dimension algebra (ses-core-build-plan §4).

#![allow(clippy::arithmetic_side_effects)]

use crate::dim::Dim;

pub mod markers;
mod qty;
mod sealed {
    /// Sealed supertrait for [`super::DimType`] (Vocabulary §1.3).
    pub trait Sealed {}
}
mod table_div;
mod table_identity;
mod table_mul;

pub use qty::Qty;

/// Marker tying a type-level dimension to its runtime [`Dim`].
pub trait DimType: sealed::Sealed {
    /// Runtime dimension value.
    const DIM: Dim;
    /// Short name for errors and debugging.
    const NAME: &'static str;
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
