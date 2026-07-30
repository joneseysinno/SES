//! Inert exact rational for storage (Vocabulary §1.3).
//!
//! Representation and ordering only — general arithmetic lives in `ses-engineer`.

/// Exact rational in reduced form.
///
/// Invariants: `den > 0`, `gcd(|num|, den) == 1`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rational {
    pub(crate) num: i64,
    pub(crate) den: i64,
}

mod add;
mod checked_neg;
mod default;
mod deserialize;
mod detail;
mod display;
mod div;
mod from_decimal_str;
mod from_int;
mod is_integer;
mod mul;
mod new;
mod one;
mod ord;
mod serialize;
mod zero;

#[path = "rational/den.rs"]
mod den_method;
#[path = "rational/num.rs"]
mod num_method;

pub(crate) use detail::{parse_i64, pow10_i64, reduce};
