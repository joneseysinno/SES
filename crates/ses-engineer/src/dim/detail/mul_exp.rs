use crate::repr_error::DimError;

pub(crate) fn mul_exp(lhs: i8, rhs: i8, axis: &'static str) -> Result<i8, DimError> {
    lhs.checked_mul(rhs)
        .ok_or(DimError::ExponentOverflow { axis, lhs, rhs })
}
