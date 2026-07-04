//! Pure-representation types shared across the SES workspace.
//!
//! Zero runtime dependencies — no arithmetic tower, no serde, no InfiniteDB.
//! See `plans/ses-core-build-plan.md` for scope and non-goals.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![deny(clippy::arithmetic_side_effects)]

pub mod authored;
pub mod dim;
pub mod dimtype;
pub mod error;
pub mod id;
pub mod rational;

pub use authored::{parse_authored, render, Authored, UnitSym};
pub use dim::Dim;
pub use dimtype::{DimType, Qty};
pub use error::{AuthoredErrorKind, AuthoredParseError, DimError, RationalError};
pub use id::UnitId;
pub use rational::Rational;
