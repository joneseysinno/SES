# ses-engineer — Engineering Substrate

`ses-engineer` is the **complete engineering substrate** for the SES workspace:
dimensions, exact rationals, units, quantities, authored grammar, tower
arithmetic, conversion, certified comparison, and rounding.

It depends on `ses-core` for constitutional markers (`Ephemeral`, `Policy`,
`Decided`) and on `adele-ring` for exact tower arithmetic.

## Modules

| Module | Role | Spec |
|--------|------|------|
| `dim` / `dimtype` | Compile-time dimension algebra | Vocabulary §1.3 |
| `rational` | Exact ℚ storage | Vocabulary §1.3 |
| `unit` | Registry, symbols, imperial seed | Vocabulary §1.3 |
| `quantity` | Authored persisted magnitudes | Vocabulary §1.1 |
| `measure` | Ephemeral derived magnitudes | Vocabulary §1.3 |
| `convert` | Point-of-use unit conversion | Vocabulary §1.3 |
| `tower` | Adele-ring lift/narrow | Pipeline §7 |
| `compare` | Certified ordering | Vocabulary §1.3 |
| `round` | Single serialization rounding event | Vocabulary §1.3 |
| `authored` | Engineer notation parser | Vocabulary §1.3 |
| `expr` | Provision expression AST | Code-pipeline §6 |

## Testimony doctrine

- `Quantity` carries **authored** testimony — engineer-entered values with
  verbatim origin strings.
- `Measure` and `UnitRegistry` are **ephemeral** — derived at point of use,
  never persisted as testimony.
- `convert` returns `Measure`, never `Quantity`.

## Dependencies

Runtime: `ses-core`, `adele-ring`, `serde`, `thiserror` only.
