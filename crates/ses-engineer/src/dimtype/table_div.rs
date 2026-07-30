use super::DimDiv;
use super::markers::*;

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

#[cfg(test)]
macro_rules! dim_div_test {
    ($a:ident, $b:ident, $out:ident) => {
        paste::paste! {
            #[test]
            fn [<dim_div_ $a _ $b _is_ $out>]() {
                assert_eq!(
                    <$a as DimType>::DIM.div(<$b as DimType>::DIM).unwrap(),
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

    dim_div_test!(F1, L1, FperL);
    dim_div_test!(F1, L2, Stress);
    dim_div_test!(F1, L3, FperL3);
    dim_div_test!(FL, L1, F1);
    dim_div_test!(FL, F1, L1);
}
