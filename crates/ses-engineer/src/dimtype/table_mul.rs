use super::DimMul;
use super::markers::*;

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

#[cfg(test)]
macro_rules! dim_mul_test {
    ($a:ident, $b:ident, $out:ident) => {
        paste::paste! {
            #[test]
            fn [<dim_mul_ $a _ $b _is_ $out>]() {
                assert_eq!(
                    <$a as DimType>::DIM.mul(<$b as DimType>::DIM).unwrap(),
                    <$out as DimType>::DIM
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

    dim_mul_test!(L1, L1, L2);
    dim_mul_test!(L2, L1, L3);
    dim_mul_test!(L3, L1, L4);
    dim_mul_test!(F1, L1, FL);
    dim_mul_test!(L1, F1, FL);
    dim_mul_test!(Stress, L2, F1);
    dim_mul_test!(L2, Stress, F1);
    dim_mul_test!(Stress, L1, FperL);
    dim_mul_test!(L1, Stress, FperL);
}
