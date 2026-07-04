use crate::repr_error::RationalError;

pub(crate) fn parse_i64(s: &str) -> Result<i64, RationalError> {
    s.parse::<i64>().map_err(|_| RationalError::ParseError {
        input: s.to_string(),
        reason: "invalid integer",
    })
}
