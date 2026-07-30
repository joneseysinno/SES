use crate::authored::Authored;
use crate::error::EngineerError;
use crate::rational::Rational;
use crate::unit::{UnitId, UnitRegistry};

use super::Quantity;

/// Evaluate authored AST against the registry (Vocabulary §1.3).
///
/// Feet-inches reduce to inches; products multiply out; the verbatim string is
/// preserved as [`Quantity::authored`].
pub fn from_authored(
    ast: &Authored,
    registry: &UnitRegistry,
    authored: impl Into<String>,
) -> Result<Quantity, EngineerError> {
    let (value, unit) = eval(ast, registry)?;
    Ok(Quantity::new(value, unit, authored))
}

fn eval(ast: &Authored, registry: &UnitRegistry) -> Result<(Rational, UnitId), EngineerError> {
    match ast {
        Authored::Quantity { value, unit } => {
            if let Some(sym) = unit {
                let unit_id = resolve_unit(sym, registry)?;
                Ok((*value, unit_id))
            } else {
                Err(EngineerError::UnknownSymbol("unit".into()))
            }
        }
        Authored::FeetInches { feet, inches } => {
            let inch_unit = registry
                .get_by_symbol("in")
                .ok_or_else(|| EngineerError::UnknownSymbol("in".into()))?
                .id;
            let total_inches = feet
                .mul(Rational::from_int(12))?
                .add(inches.unwrap_or(Rational::from_int(0)))?;
            Ok((total_inches, inch_unit))
        }
        Authored::Product(items) => eval_product(items, registry),
    }
}

fn eval_factor(
    ast: &Authored,
    registry: &UnitRegistry,
) -> Result<(Rational, Option<UnitId>), EngineerError> {
    match ast {
        Authored::Quantity { value, unit } => {
            let unit_id = unit
                .as_ref()
                .map(|sym| resolve_unit(sym, registry))
                .transpose()?;
            Ok((*value, unit_id))
        }
        Authored::FeetInches { .. } => {
            let (value, unit) = eval(ast, registry)?;
            Ok((value, Some(unit)))
        }
        Authored::Product(items) => {
            let (value, unit) = eval_product(items, registry)?;
            Ok((value, Some(unit)))
        }
    }
}

fn eval_product(
    items: &[Authored],
    registry: &UnitRegistry,
) -> Result<(Rational, UnitId), EngineerError> {
    let mut value = Rational::from_int(1);
    let mut unit: Option<UnitId> = None;
    let mut dim = crate::Dim::DIMENSIONLESS;

    for item in items {
        let (item_value, item_unit) = eval_factor(item, registry)?;
        let Some(item_unit) = item_unit else {
            value = value.mul(item_value)?;
            continue;
        };
        let entry = registry
            .get(item_unit)
            .ok_or(EngineerError::UnknownUnit(item_unit))?;

        match unit {
            None => {
                value = value.mul(item_value)?;
                unit = Some(item_unit);
                dim = entry.dim;
            }
            Some(_current_unit) if entry.dim == crate::Dim::DIMENSIONLESS => {
                value = value.mul(item_value)?;
            }
            Some(_current_unit) if dim == crate::Dim::DIMENSIONLESS => {
                value = item_value.mul(value)?;
                unit = Some(item_unit);
                dim = entry.dim;
            }
            Some(current_unit) => {
                let current_entry = registry
                    .get(current_unit)
                    .ok_or(EngineerError::UnknownUnit(current_unit))?;
                let pivot_value = value
                    .mul(current_entry.ratio_to_pivot)?
                    .mul(item_value.mul(entry.ratio_to_pivot)?)?;
                dim = dim.mul(entry.dim).map_err(EngineerError::from)?;
                let pivot_unit = find_unit_for_dim(registry, dim)?;
                let pivot_entry = registry
                    .get(pivot_unit)
                    .ok_or(EngineerError::UnknownUnit(pivot_unit))?;
                value = pivot_value.div(pivot_entry.ratio_to_pivot)?;
                unit = Some(pivot_unit);
            }
        }
    }

    let unit = unit.ok_or_else(|| EngineerError::UnknownSymbol("unit".into()))?;
    Ok((value, unit))
}

fn resolve_unit(
    sym: &crate::authored::UnitSym,
    registry: &UnitRegistry,
) -> Result<UnitId, EngineerError> {
    registry
        .get_by_symbol(&sym.0)
        .map(|entry| entry.id)
        .ok_or_else(|| EngineerError::UnknownSymbol(sym.0.clone()))
}

fn find_unit_for_dim(registry: &UnitRegistry, dim: crate::Dim) -> Result<UnitId, EngineerError> {
    registry
        .entries()
        .iter()
        .find(|entry| entry.dim == dim && entry.ratio_to_pivot == Rational::from_int(1))
        .or_else(|| registry.entries().iter().find(|entry| entry.dim == dim))
        .map(|entry| entry.id)
        .ok_or_else(|| EngineerError::UnknownSymbol(format!("{dim:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authored::parse_authored;

    #[test]
    fn product_multiplies_out() {
        let reg = crate::unit::imperial_seed();
        let ast = parse_authored("3 × 8 ft").unwrap();
        let q = from_authored(&ast, &reg, "3 × 8 ft").unwrap();
        assert_eq!(q.value, Rational::from_int(24));
        assert_eq!(q.unit, UnitId(1));
    }
}
