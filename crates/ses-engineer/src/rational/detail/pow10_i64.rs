use crate::repr_error::RationalError;

pub(crate) fn pow10_i64(exp: u32) -> Result<i64, RationalError> {
    let mut v: i64 = 1;
    for _ in 0..exp {
        v = v.checked_mul(10).ok_or(RationalError::Overflow)?;
    }
    Ok(v)
}
