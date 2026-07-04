use crate::error::EngineerError;
use crate::measure::Measure;
use crate::quantity::Quantity;
use crate::tower::lift;
use crate::unit::{UnitId, UnitRegistry};

use super::ratio_between::ratio_between;

/// Point-of-use conversion returning ephemeral `Measure` (Vocabulary §1.3).
///
/// Derived state must not carry authored testimony — conversion is computed
/// at the point of use and stored in tower arithmetic.
pub fn convert(
    q: &Quantity,
    target: UnitId,
    registry: &UnitRegistry,
) -> Result<Measure, EngineerError> {
    if q.unit == target {
        return Ok(Measure {
            value: lift(q.value),
            unit: target,
        });
    }

    let ratio = ratio_between(q.unit, target, registry)?;
    let scaled = lift(q.value).mul(&lift(ratio));
    Ok(Measure {
        value: scaled,
        unit: target,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authored::parse_authored;
    use crate::quantity::from_authored;
    use crate::rational::Rational;
    use crate::tower::narrow;
    use crate::unit::imperial_seed;

    #[test]
    fn twenty_four_ft_to_inches() {
        let reg = imperial_seed();
        let q = from_authored(&parse_authored("24 ft").unwrap(), &reg, "24 ft").unwrap();
        let m = convert(&q, UnitId(0), &reg).unwrap();
        assert_eq!(narrow(m.value).unwrap(), Rational::from_int(288));
        assert_eq!(m.unit, UnitId(0));
    }

    #[test]
    fn feet_inches_fraction_to_inches() {
        let reg = imperial_seed();
        let q = from_authored(&parse_authored("3'-6 1/2\"").unwrap(), &reg, "3'-6 1/2\"").unwrap();
        let m = convert(&q, UnitId(0), &reg).unwrap();
        assert_eq!(narrow(m.value).unwrap(), Rational::new(85, 2).unwrap());
    }

    #[test]
    fn round_trip_ft_in_value_identical() {
        let reg = imperial_seed();
        let q = from_authored(&parse_authored("24 ft").unwrap(), &reg, "24 ft").unwrap();
        let in_measure = convert(&q, UnitId(0), &reg).unwrap();
        let back = convert(
            &Quantity::new(narrow(in_measure.value).unwrap(), in_measure.unit, "288 in"),
            UnitId(1),
            &reg,
        )
        .unwrap();
        assert_eq!(back.value, lift(q.value));
        assert_eq!(back.unit, UnitId(1));
    }
}
