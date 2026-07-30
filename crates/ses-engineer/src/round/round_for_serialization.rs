use crate::rational::Rational;

use super::RoundingPolicyV1;

/// Apply rounding policy for serialization (Vocabulary §1.3).
pub fn round_for_serialization(value: Rational, policy: RoundingPolicyV1) -> Rational {
    let _ = policy;
    let places = RoundingPolicyV1::DECIMAL_PLACES;
    let scale = pow10_i64(places);
    let scaled = value
        .mul(Rational::from_int(scale))
        .expect("scale is non-zero");
    let rounded_num = div_round_half_away_from_zero(scaled.num(), scaled.den());
    Rational::from_int(rounded_num)
        .div(Rational::from_int(scale))
        .expect("scale is non-zero")
}

fn pow10_i64(exp: u32) -> i64 {
    10i64.pow(exp)
}

fn div_round_half_away_from_zero(num: i64, den: i64) -> i64 {
    if den == 0 {
        return num;
    }
    let (quot, rem) = (num / den, num % den);
    if rem.abs() * 2 >= den.abs() {
        quot + num.signum() * den.signum()
    } else {
        quot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_platform_determinism() {
        let value = Rational::new(1, 3).unwrap();
        let once = round_for_serialization(value, RoundingPolicyV1);
        let twice = round_for_serialization(value, RoundingPolicyV1);
        assert_eq!(once, twice);
        assert_eq!(once, Rational::new(333_333, 1_000_000).unwrap());
    }
}
