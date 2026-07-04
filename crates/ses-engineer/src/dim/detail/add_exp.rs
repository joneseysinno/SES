use crate::repr_error::DimError;

pub(crate) fn add_exp(lhs: i8, rhs: i8, axis: &'static str) -> Result<i8, DimError> {
    lhs.checked_add(rhs)
        .ok_or(DimError::ExponentOverflow { axis, lhs, rhs })
}
