pub(crate) fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[cfg(test)]
mod proptests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use crate::Rational;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn new_invariants(num in 1i64..10_000, den in 1i64..10_000) {
            let r = Rational::new(num, den).unwrap();
            prop_assert!(r.den() > 0);
            prop_assert_eq!(gcd_i128(r.num().abs() as i128, r.den() as i128), 1);
        }
    }
}
