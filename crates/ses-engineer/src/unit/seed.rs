use crate::dim::Dim;
use crate::rational::Rational;
use crate::unit::{UnitEntry, UnitId, UnitRegistry, UnitSystem};

/// Built-in imperial unit rows with exact pivot ratios (Vocabulary §1.3).
///
/// Pivots: in (L), lbf (F), psi (F·L⁻²), lbf·in (F·L), pcf (F·L⁻³), s (T), degF (Θ).
pub fn imperial_seed() -> UnitRegistry {
    let mut registry = UnitRegistry::new();
    for entry in imperial_rows() {
        registry.register(entry);
    }
    registry
}

fn row(id: u32, symbol: &str, name: &str, dim: Dim, ratio_num: i64, ratio_den: i64) -> UnitEntry {
    UnitEntry {
        id: UnitId(id),
        symbol: symbol.to_string(),
        name: name.to_string(),
        dim,
        ratio_to_pivot: Rational::from_int(ratio_num)
            .div(Rational::from_int(ratio_den))
            .expect("seed ratio denominator is non-zero"),
        system: UnitSystem::Imperial,
    }
}

fn imperial_rows() -> Vec<UnitEntry> {
    vec![
        row(0, "in", "inch", Dim::LENGTH, 1, 1),
        row(1, "ft", "foot", Dim::LENGTH, 12, 1),
        row(2, "yd", "yard", Dim::LENGTH, 36, 1),
        row(3, "lbf", "pound-force", Dim::FORCE, 1, 1),
        row(4, "kip", "kip", Dim::FORCE, 1000, 1),
        row(5, "psi", "pound per square inch", Dim::STRESS, 1, 1),
        row(6, "ksi", "kip per square inch", Dim::STRESS, 1000, 1),
        row(7, "psf", "pound per square foot", Dim::STRESS, 1, 144),
        row(8, "ksf", "kip per square foot", Dim::STRESS, 1000, 144),
        row(9, "lbf-in", "pound-inch", Dim::MOMENT, 1, 1),
        row(10, "lbf-ft", "pound-foot", Dim::MOMENT, 12, 1),
        row(11, "kip-in", "kip-inch", Dim::MOMENT, 1000, 1),
        row(12, "kip-ft", "kip-foot", Dim::MOMENT, 12000, 1),
        row(
            13,
            "pcf",
            "pound per cubic foot",
            Dim::FORCE_PER_VOLUME,
            1,
            1,
        ),
        row(14, "s", "second", Dim::TIME, 1, 1),
        row(15, "degF", "degree Fahrenheit interval", Dim::TEMP, 1, 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::UnitId;

    #[test]
    fn seed_has_expected_count() {
        assert_eq!(imperial_seed().entries().len(), 16);
    }

    #[test]
    fn inch_is_pivot_length() {
        let reg = imperial_seed();
        let inch = reg.get(UnitId(0)).expect("inch");
        assert_eq!(inch.ratio_to_pivot, Rational::from_int(1));
    }
}
