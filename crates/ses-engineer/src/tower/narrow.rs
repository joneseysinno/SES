use adele_ring::tower::{TowerLevel, TowerValue as ArTowerValue};

use crate::error::EngineerError;
use crate::rational::Rational;

use super::TowerValue;

fn parse_display(s: &str) -> Result<Rational, EngineerError> {
    if let Some((num, den)) = s.split_once('/') {
        let num: i64 = num.parse().map_err(|_| EngineerError::Overflow)?;
        let den: i64 = den.parse().map_err(|_| EngineerError::Overflow)?;
        Rational::new(num, den).map_err(|_| EngineerError::Overflow)
    } else {
        let num: i64 = s.parse().map_err(|_| EngineerError::Overflow)?;
        Ok(Rational::from_int(num))
    }
}

fn narrow_inner(value: &ArTowerValue) -> Result<Rational, EngineerError> {
    match value {
        ArTowerValue::Rational(r) => parse_display(&r.display()),
        ArTowerValue::Integer(i) => {
            let exact = ArTowerValue::Integer(i.clone()).digits(0);
            parse_display(&exact.display())
        }
        ArTowerValue::Algebraic(a) => {
            if let Some(r) = a.to_rational() {
                return parse_display(&r.display());
            }
            let lowered = ArTowerValue::Algebraic(a.clone()).reduce();
            if !matches!(lowered, ArTowerValue::Algebraic(_)) {
                return narrow_inner(&lowered);
            }
            Err(EngineerError::Overflow)
        }
        _ => Err(EngineerError::Overflow),
    }
}

/// Narrow a tower value back to ℚ (Vocabulary §1.3).
pub fn narrow(value: TowerValue) -> Result<Rational, EngineerError> {
    let reduced = value.0.reduce();
    if matches!(
        reduced.level(),
        TowerLevel::Integer | TowerLevel::Rational | TowerLevel::Algebraic
    ) {
        return narrow_inner(&reduced);
    }
    Err(EngineerError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::lift;

    #[test]
    fn lift_narrow_round_trip() {
        let r = Rational::from_int(288);
        let lifted = lift(r);
        assert_eq!(narrow(lifted).unwrap(), r);
    }

    #[test]
    fn algebraic_sqrt_cannot_narrow() {
        let inner = adele_ring::TowerValue::sqrt(&lift(Rational::from_int(2)).0);
        assert!(matches!(
            narrow(TowerValue(inner)),
            Err(EngineerError::Overflow)
        ));
    }
}
