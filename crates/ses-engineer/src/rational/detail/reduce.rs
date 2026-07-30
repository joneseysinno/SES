use crate::repr_error::RationalError;

use crate::Rational;

use super::gcd_i128;

pub(crate) fn reduce(num: i64, den: i64) -> Result<Rational, RationalError> {
    if den == 0 {
        return Err(RationalError::ZeroDenominator);
    }
    let (mut num, mut den) = (num, den);
    if den < 0 {
        num = num.checked_neg().ok_or(RationalError::Overflow)?;
        den = den.checked_neg().ok_or(RationalError::Overflow)?;
    }
    let num_abs_i128: i128 = match num.checked_abs() {
        Some(v) => i128::from(v),
        None => 1i128 << 63,
    };
    let g = gcd_i128(num_abs_i128, i128::from(den)) as i64;
    let num_reduced = num / g;
    let den_reduced = den / g;
    Ok(Rational {
        num: num_reduced,
        den: den_reduced,
    })
}
