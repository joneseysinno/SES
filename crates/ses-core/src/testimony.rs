//! Testimony doctrine (Vocabulary §1.1).
//!
//! Every value in SES belongs to exactly one epistemic domain:
//!
//! | Domain | Origin | Persists | Marker |
//! |--------|--------|----------|--------|
//! | Authored | Engineer entry | Yes | [`Testimony`] |
//! | Emitted | Engine run | Yes | [`Testimony`] |
//! | Imported | External catalog | Yes | [`Testimony`] |
//! | Derived | Point-of-use computation | No | [`Ephemeral`] |
//!
//! Persisted payloads implement [`Testimony`]; derived computation implements
//! [`Ephemeral`]. The trichotomy forbids mixing domains.

mod ephemeral;
mod kind;
mod testimony_trait;

pub use ephemeral::Ephemeral;
pub use kind::TestimonyKind;
pub use testimony_trait::Testimony;
