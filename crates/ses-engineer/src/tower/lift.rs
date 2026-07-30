use adele_ring::{Basis, RnsRational, TowerValue as ArTowerValue};

use crate::rational::Rational;

use super::TowerValue;

/// Lift an exact rational into the tower (Vocabulary §1.3).
pub fn lift(value: Rational) -> TowerValue {
    let basis = Basis::standard();
    let inner = RnsRational::from_fraction(value.num(), value.den(), basis);
    TowerValue(ArTowerValue::Rational(inner))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::narrow;

    #[test]
    fn lift_narrow_round_trip() {
        let r = Rational::from_int(288);
        let lifted = lift(r);
        assert_eq!(narrow(lifted).unwrap(), r);
    }
}
