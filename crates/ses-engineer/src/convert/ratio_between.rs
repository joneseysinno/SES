use crate::error::EngineerError;
use crate::rational::Rational;
use crate::unit::{UnitId, UnitRegistry};

/// Exact pivot-ratio `from / to` with dimension equality checked first (Vocabulary §1.3).
pub fn ratio_between(
    from: UnitId,
    to: UnitId,
    registry: &UnitRegistry,
) -> Result<Rational, EngineerError> {
    let from_entry = registry.get(from).ok_or(EngineerError::UnknownUnit(from))?;
    let to_entry = registry.get(to).ok_or(EngineerError::UnknownUnit(to))?;

    if from_entry.dim != to_entry.dim {
        return Err(EngineerError::DimensionMismatch {
            expected: to_entry.dim,
            found: from_entry.dim,
        });
    }

    from_entry
        .ratio_to_pivot
        .div(to_entry.ratio_to_pivot)
        .map_err(|_| EngineerError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::imperial_seed;

    #[test]
    fn ft_to_in_ratio() {
        let reg = imperial_seed();
        let ratio = ratio_between(UnitId(1), UnitId(0), &reg).unwrap();
        assert_eq!(ratio, Rational::from_int(12));
    }

    #[test]
    fn ft_to_psi_is_dimension_mismatch() {
        let reg = imperial_seed();
        let err = ratio_between(UnitId(1), UnitId(5), &reg).unwrap_err();
        assert!(matches!(err, EngineerError::DimensionMismatch { .. }));
    }
}
