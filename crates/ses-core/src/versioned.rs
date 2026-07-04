//! Version lineage (Vocabulary §1.2).
//!
//! Persisted records carry a schema version byte. Older bytes decode through
//! the [`Lineage`] chain via `From` decoders — interpretation, not migration.

mod lineage;
mod lineage_is_contiguous;
mod root;
mod versioned_trait;

pub use lineage::{FromPrev, Genesis, Lineage, Via};
pub use lineage_is_contiguous::lineage_is_contiguous;
pub use root::Root;
pub use versioned_trait::Versioned;
