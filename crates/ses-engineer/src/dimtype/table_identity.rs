use super::markers::*;
use super::{DimDiv, DimMul};

impl DimMul<NoDim> for NoDim {
    type Out = NoDim;
}

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

dim_identity_div!(NoDim);
dim_identity_div!(L1);
dim_identity_div!(L2);
dim_identity_div!(L3);
dim_identity_div!(L4);
dim_identity_div!(InvL);
dim_identity_div!(F1);
dim_identity_div!(FL);
dim_identity_div!(FperL);
dim_identity_div!(Stress);
dim_identity_div!(FperL3);
dim_identity_div!(T1);
dim_identity_div!(Theta);

#[cfg(test)]
macro_rules! dim_identity_mul_test {
    ($x:ident) => {
        paste::paste! {
            #[test]
            fn [<dim_identity_mul_ $x _from_nodim>]() {
                assert_eq!(
                    <$x as DimType>::DIM.mul(<NoDim as DimType>::DIM).unwrap(),
                    <$x as DimType>::DIM
                );
            }

            #[test]
            fn [<dim_identity_mul_nodim_to_ $x>]() {
                assert_eq!(
                    <NoDim as DimType>::DIM.mul(<$x as DimType>::DIM).unwrap(),
                    <$x as DimType>::DIM
                );
            }
        }
    };
}

#[cfg(test)]
macro_rules! dim_identity_div_test {
    ($x:ident) => {
        paste::paste! {
            #[test]
            fn [<dim_identity_div_ $x _self_is_nodim>]() {
                assert_eq!(
                    <$x as DimType>::DIM.div(<$x as DimType>::DIM).unwrap(),
                    <NoDim as DimType>::DIM
                );
            }
        }
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod consistency {
    use super::*;
    use crate::dimtype::DimType;

    dim_identity_mul_test!(L1);
    dim_identity_mul_test!(L2);
    dim_identity_mul_test!(L3);
    dim_identity_mul_test!(L4);
    dim_identity_mul_test!(InvL);
    dim_identity_mul_test!(F1);
    dim_identity_mul_test!(FL);
    dim_identity_mul_test!(FperL);
    dim_identity_mul_test!(Stress);
    dim_identity_mul_test!(FperL3);
    dim_identity_mul_test!(T1);
    dim_identity_mul_test!(Theta);

    dim_identity_div_test!(NoDim);
    dim_identity_div_test!(L1);
    dim_identity_div_test!(L2);
    dim_identity_div_test!(L3);
    dim_identity_div_test!(L4);
    dim_identity_div_test!(InvL);
    dim_identity_div_test!(F1);
    dim_identity_div_test!(FL);
    dim_identity_div_test!(FperL);
    dim_identity_div_test!(Stress);
    dim_identity_div_test!(FperL3);
    dim_identity_div_test!(T1);
    dim_identity_div_test!(Theta);

    #[test]
    fn nodim_mul_nodim() {
        assert_eq!(
            <NoDim as DimType>::DIM
                .mul(<NoDim as DimType>::DIM)
                .unwrap(),
            <NoDim as DimType>::DIM
        );
    }
}
