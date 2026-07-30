//! Engineering domain types: quantities, units, and provision expressions.
//!
//! # End-to-end tower pipeline
//!
//! ```
//! use ses_engineer::{
//!     certified_cmp, from_authored, imperial_seed, lift, narrow, parse_authored,
//!     round_for_serialization, sqrt_psi, Rational, RoundingPolicyV1, Stress,
//! };
//! use core::cmp::Ordering;
//!
//! let registry = imperial_seed();
//! let q = from_authored(
//!     &parse_authored("2500 psi").unwrap(),
//!     &registry,
//!     "2500 psi",
//! )
//! .unwrap();
//! let rooted = sqrt_psi(Stress::new(lift(q.value))).unwrap();
//! let fifty = lift(Rational::from_int(50));
//! let decided = certified_cmp(rooted.get().clone(), fifty.clone()).unwrap();
//! assert_eq!(decided.value, Ordering::Equal);
//! let display = round_for_serialization(
//!     narrow(fifty).unwrap(),
//!     RoundingPolicyV1,
//! );
//! assert_eq!(display, Rational::from_int(50));
//! ```

pub mod authored;
pub mod compare;
pub mod convert;
pub mod dim;
pub mod dimtype;
pub mod error;
pub mod expr;
pub mod measure;
pub mod ops;
pub mod quantity;
pub mod rational;
pub mod repr_error;
pub mod round;
pub mod tower;
pub mod unit;

pub use authored::{Authored, UnitSym, parse_authored, render};
pub use compare::{CertifiedBy, certified_cmp};
pub use convert::convert;
pub use dim::Dim;
pub use dimtype::{DimType, Qty};
pub use error::EngineerError;
pub use measure::Measure;
pub use ops::sqrt_psi;
pub use quantity::{Quantity, from_authored};
pub use rational::Rational;
pub use repr_error::{AuthoredErrorKind, AuthoredParseError, DimError, RationalError};
pub use round::{RoundingPolicyV1, round_for_serialization};
pub use ses_core::Ephemeral;
pub use tower::{Area, Force, Length, LineLoad, Moment, Ratio, Stress, TowerValue, lift, narrow};
pub use unit::{UnitEntry, UnitId, UnitRegistry, UnitSystem, imperial_seed};

/// Backward-compatible alias for the runtime dimension type.
#[deprecated(note = "use Dim directly")]
pub type DimensionSignature = Dim;
