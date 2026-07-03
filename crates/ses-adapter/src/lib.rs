//! InfiniteDB adapter: spaces, payloads, hyperedge catalog, and codec.

pub mod catalog;
pub mod codec;
pub mod error;
pub mod payload;
pub mod schema;

pub use catalog::build_kind_catalog;
pub use codec::{decode_payload, encode_payload, SesPayload};
pub use error::AdapterError;
