//! InfiniteDB adapter: spaces, payloads, hyperedge catalog, and codec.

pub mod catalog;
pub mod codec;
pub mod error;
pub mod payload;
pub mod schema;

pub use catalog::{build_kind_catalog, validate_schema_names};
pub use codec::{SesPayload, decode_payload, encode_payload};
pub use error::AdapterError;
