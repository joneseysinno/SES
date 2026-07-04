# ses-core — The SES Constitution

`ses-core` is the **constitution crate** for the SES workspace: testimony,
versioning, provenance, decision, policy, and naming law. It has **zero
runtime dependencies** and contains **no domain nouns** — no dimensions,
quantities, units, or engineering representation.

Engineering types live in `ses-engineer`. Adapter, analysis, and UI crates
import `ses-core` for epistemic markers and naming validators; they import
`ses-engineer` for computation.

## Domains

| Module | Question | Key types |
|--------|----------|-----------|
| `testimony` | What is the epistemic status? | `Testimony`, `Ephemeral`, `TestimonyKind` |
| `versioned` | Which schema generation? | `Versioned`, `Lineage`, `Root` |
| `provenance` | Which upstream revision? | `Pin`, `Revision`, `Provenanced` |
| `decision` | What was decided and why? | `Decided<T, J>` |
| `policy` | Which norms apply? | `Policy`, `policies()` |
| `convention` | Does this name obey naming law? | `validate_*`, `ConventionError` |

## Testimony doctrine (Vocabulary §1.1)

Every value belongs to exactly one epistemic domain:

| Domain | Origin | Persists | Marker |
|--------|--------|----------|--------|
| Authored | Engineer entry | Yes | `Testimony` |
| Emitted | Engine run | Yes | `Testimony` |
| Imported | External catalog | Yes | `Testimony` |
| Derived | Point-of-use computation | No | `Ephemeral` |

Persisted payloads implement `Testimony`. Derived computation implements
`Ephemeral`. The trichotomy forbids mixing domains — conversion returns
ephemeral values, never re-authors them.

## Version lineage (Vocabulary §1.2)

Persisted records carry a schema version byte. Older bytes decode through
the `Lineage` chain via `From` decoders — **interpretation, not migration**.
Stored bytes are never rewritten. Genesis types set `Supersedes = Root`.

## Provenance (Vocabulary §1.2)

`Pin<Id>` records which revision of an upstream record was observed.
`pin_is_stale` and `any_stale_pins` detect when derived state must be
recomputed.

## Decision (Vocabulary §1.3)

`Decided<T, J>` pairs an outcome with its justification — ordering verdicts,
policy applications, and certified comparisons all use this shape.

## Policies

Constitutional policies are zero-sized markers implementing `Policy`:

- `NoFloats` — exact arithmetic only
- `RejectUnknownKinds` — catalogs reject unknown labels at registration
- `NoSilentDefaults` — testimony types forbid `Default`
- `SingleRoundingEvent` — one rounding at the serialization boundary

Call `policies()` for the full inventory.

## Naming law (ses-vocabulary §1.1, §4–§5)

| Validator | Rule | Example |
|-----------|------|---------|
| `validate_edge_kind` | lowercase dot-separated segments | `project.contains` |
| `validate_role` | lowercase identifier | `owner` |
| `validate_counter_name` | `proj`, `edge`, or prefixed | `elem:1`, `combo:1:2` |
| `validate_space_name` | snake_case identifier | `check_results` |

Violations return `ConventionError`.

## Adding to this crate

Only cross-cutting epistemic law belongs here. If it computes, converts,
rounds, or names a physical unit, it belongs in `ses-engineer` or above.
