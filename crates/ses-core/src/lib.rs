//! The SES constitution: testimony, versioning, provenance, decision, policy,
//! and naming law.
//!
//! Zero runtime dependencies — no domain nouns, no engineering representation.
//! Engineering types live in `ses-engineer`.
//!
//! # Domains
//!
//! | Module | Question answered |
//! |--------|-------------------|
//! | [`testimony`] | What is the epistemic status of this value? |
//! | [`versioned`] | Which schema generation does this record belong to? |
//! | [`provenance`] | Which revision of upstream records was observed? |
//! | [`decision`] | What was decided, and with what justification? |
//! | [`policy`] | Which constitutional norms apply? |
//! | [`convention`] | Does this name obey naming law? |

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![deny(clippy::arithmetic_side_effects)]

pub mod convention;
pub mod decision;
pub mod policy;
pub mod provenance;
pub mod testimony;
pub mod versioned;

pub use convention::{
    ConventionError, validate_counter_name, validate_edge_kind, validate_role, validate_space_name,
};
pub use decision::Decided;
pub use policy::{Policy, PolicyEntry, policies};
pub use provenance::{Pin, Provenanced, Revision, any_stale_pins, pin_is_stale};
pub use testimony::{Ephemeral, Testimony, TestimonyKind};
pub use versioned::{FromPrev, Genesis, Lineage, Root, Versioned, Via, lineage_is_contiguous};
