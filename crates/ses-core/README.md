# ses-core

Pure-representation root of the SES workspace.

## Retained set

- **Dimension algebra** — runtime [`Dim`](src/dim.rs) (F·L·T·Θ) and compile-time [`Qty`](src/dimtype.rs)
- **Inert [`Rational`](src/rational.rs)** — storage-shaped exact fractions
- **Authored grammar** — [`parse_authored`](src/authored.rs) / [`render`](src/authored.rs)
- **Shared IDs** — [`UnitId`](src/id.rs)
- **Error taxonomy** — [`DimError`](src/error.rs), [`RationalError`](src/error.rs), [`AuthoredParseError`](src/error.rs)

## Non-goals

No unit data, conversion, comparison, rounding, tower values, serde, InfiniteDB,
payload encoding, or provision logic. Those live in `ses-engineer` and above.

## Adding to this crate

Representation, shared vocabulary, and nothing that computes. If it needs
`adele-ring`, serde, or a registry, it does not belong here.
