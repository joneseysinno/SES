use crate::dim::Dim;

use super::{DimType, sealed};

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

dim_marker!(
    NoDim,
    Dim::DIMENSIONLESS,
    "Dimensionless ratios and factors"
);
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
