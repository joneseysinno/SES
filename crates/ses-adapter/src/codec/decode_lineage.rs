use serde::de::DeserializeOwned;
use ses_core::versioned::{Lineage, Root, Versioned};

use super::{decode_genesis, decode_successor};
use crate::error::AdapterError;

/// Walks the [`Versioned`] lineage chain when decoding persisted bytes.
pub trait DecodeLineage: Versioned + Lineage + DeserializeOwned + Sized {
    /// Decode `data` (version byte + body) into the requested generation.
    fn decode_lineage(data: &[u8]) -> Result<Self, AdapterError>;
}

impl<T> DecodeLineage for T
where
    T: Versioned<Supersedes = Root> + Lineage + DeserializeOwned + Sized,
{
    fn decode_lineage(data: &[u8]) -> Result<Self, AdapterError> {
        decode_genesis::<T>(data)
    }
}

/// Explicit successor decoder — invoke from `impl DecodeLineage for V2`.
pub fn decode_from_predecessor<T, Prev>(data: &[u8]) -> Result<T, AdapterError>
where
    T: Versioned<Supersedes = Prev> + Lineage + DeserializeOwned + Sized + From<Prev>,
    Prev: DecodeLineage,
{
    decode_successor::<T, Prev>(data)
}
