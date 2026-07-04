use adele_ring::tower::{TowerLevel, TowerValue as ArTowerValue};
use adele_ring::{Basis, RnsRational};
use ses_core::Decided;

use crate::error::EngineerError;
use crate::tower::{TowerValue, narrow};

use super::certified_by::CertifiedBy;

fn parse_display_i64_pair(s: &str) -> Result<(i64, i64), EngineerError> {
    if let Some((num, den)) = s.split_once('/') {
        let num: i64 = num.parse().map_err(|_| EngineerError::Overflow)?;
        let den: i64 = den.parse().map_err(|_| EngineerError::Overflow)?;
        Ok((num, den))
    } else {
        let num: i64 = s.parse().map_err(|_| EngineerError::Overflow)?;
        Ok((num, 1))
    }
}

/// Certified ordering of two tower values (Vocabulary §1.3).
pub fn certified_cmp(
    a: TowerValue,
    b: TowerValue,
) -> Result<Decided<core::cmp::Ordering, CertifiedBy>, EngineerError> {
    if a.0.level() == TowerLevel::Rational && b.0.level() == TowerLevel::Rational {
        if let (ArTowerValue::Rational(ra), ArTowerValue::Rational(rb)) = (&a.0, &b.0) {
            return Ok(Decided::new(ra.cmp(rb), CertifiedBy::exact_rational()));
        }
    }

    let neg_one = ArTowerValue::Rational(RnsRational::from_fraction(-1, 1, Basis::standard()));
    let diff = TowerValue(a.0.add(&b.0.mul(&neg_one)));

    if let Ok(r) = narrow(diff.clone()) {
        let zero = crate::rational::Rational::from_int(0);
        return Ok(Decided::new(r.cmp(&zero), CertifiedBy::exact_rational()));
    }

    if let ArTowerValue::Algebraic(alg) = &diff.0 {
        let sign = alg.sign();
        let ord = sign.cmp(&0);
        let (lo, hi) = &alg.interval;
        let width = hi.sub(lo);
        let (width_num, width_den) = parse_display_i64_pair(&width.display())?;
        return Ok(Decided::new(
            ord,
            CertifiedBy::interval(width_num, width_den),
        ));
    }

    Err(EngineerError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rational::Rational;
    use crate::tower::lift;

    #[test]
    fn rational_ordering_is_exact() {
        let a = lift(Rational::from_int(3));
        let b = lift(Rational::from_int(5));
        let decided = certified_cmp(a, b).unwrap();
        assert_eq!(decided.value, core::cmp::Ordering::Less);
        assert_eq!(decided.justification, CertifiedBy::exact_rational());
    }
}
