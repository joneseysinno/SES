use crate::repr_error::AuthoredErrorKind;

use super::detail::{contains_spaced_x, err};
use super::parse_product::{parse_product, parse_single};
use crate::authored::Authored;

/// Parse engineer-authored quantity text.
pub fn parse_authored(input: &str) -> Result<Authored, crate::repr_error::AuthoredParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(err(0, AuthoredErrorKind::EmptyInput));
    }
    if trimmed.contains('×') || contains_spaced_x(trimmed) {
        return parse_product(trimmed);
    }
    parse_single(trimmed, 0)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use crate::authored::{Authored, render};
    use crate::rational::Rational;

    fn parse(s: &str) -> Authored {
        parse_authored(s).expect("should parse")
    }

    #[test]
    fn quantity_with_unit() {
        let ast = parse("24 ft");
        assert!(matches!(ast, Authored::Quantity { .. }));
    }

    #[test]
    fn feet_inches() {
        let ast = parse("3'-6\"");
        assert!(matches!(ast, Authored::FeetInches { .. }));
    }

    #[test]
    fn feet_inches_fraction() {
        parse("3'-6 1/2\"");
    }

    #[test]
    fn feet_only() {
        parse("3'");
    }

    #[test]
    fn inches_only() {
        parse("6\"");
    }

    #[test]
    fn mixed_number() {
        parse("5 1/2 in");
    }

    #[test]
    fn product_times() {
        parse("3 × 8 ft");
    }

    #[test]
    fn product_x() {
        parse("2 x 12 in");
    }

    #[test]
    fn decimal_dimensionless() {
        let ast = parse("0.0025");
        if let Authored::Quantity { value, unit } = ast {
            assert_eq!(value, Rational::from_decimal_str("0.0025").unwrap());
            assert!(unit.is_none());
        } else {
            panic!("expected quantity");
        }
    }

    #[test]
    fn stress_unit() {
        parse("5000 psi");
        parse("1.5 kip/ft");
    }

    #[test]
    fn reject_ambiguous() {
        assert!(parse_authored("3x8").is_err());
    }

    #[test]
    fn reject_empty() {
        assert!(parse_authored("").is_err());
    }

    #[test]
    fn reject_unit_before_qty() {
        assert!(parse_authored("ft 24").is_err());
    }

    #[test]
    fn reject_trailing_separator() {
        assert!(parse_authored("3'-").is_err());
    }

    #[test]
    fn reject_zero_fraction() {
        assert!(parse_authored("1/0 in").is_err());
    }

    #[test]
    fn round_trip_idempotent() {
        for s in ["24 ft", "3'-6\"", "0.0025", "5000 psi", "3 × 8 ft"] {
            let once = parse_authored(s).unwrap();
            let rendered = render(&once);
            let twice = parse_authored(&rendered).unwrap();
            let rerendered = render(&twice);
            assert_eq!(render(&once), rerendered, "round-trip failed for {s}");
        }
    }

    #[test]
    fn never_panics_on_adversarial_inputs() {
        let cases = [
            "",
            "x",
            "xxx",
            "3x8",
            "''''",
            " × ",
            &"x ".repeat(64),
            "3'-6\"'-6\"",
        ];
        for s in cases {
            let _ = parse_authored(s);
        }
    }
}
