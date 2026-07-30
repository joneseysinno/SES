//! Conversion property and integration tests (Phase 4).

use proptest::prelude::*;
use ses_engineer::{
    EngineerError, Quantity, Rational, convert, imperial_seed, narrow, unit::UnitId,
};

fn registry() -> ses_engineer::unit::UnitRegistry {
    imperial_seed()
}

proptest! {
    #[test]
    fn conversion_through_intermediate_equals_direct(
        feet in 1i64..1000,
    ) {
        let reg = registry();
        let q = Quantity::new(Rational::from_int(feet), UnitId(1), format!("{feet} ft"));
        let direct = convert(&q, UnitId(0), &reg).unwrap();
        let via_yd = convert(&q, UnitId(2), &reg).unwrap();
        let from_yd = convert(
            &Quantity::new(
                narrow(via_yd.value.clone()).unwrap(),
                via_yd.unit,
                "via yd",
            ),
            UnitId(0),
            &reg,
        )
        .unwrap();
        prop_assert_eq!(direct.value, from_yd.value);
    }
}

#[test]
fn ft_to_psi_is_dimension_mismatch() {
    let reg = registry();
    let q = Quantity::new(Rational::from_int(1), UnitId(1), "1 ft");
    let err = convert(&q, UnitId(5), &reg).unwrap_err();
    assert!(matches!(err, EngineerError::DimensionMismatch { .. }));
}
