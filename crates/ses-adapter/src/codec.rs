use bincode::config::standard;
use bincode::serde::{decode_from_slice, encode_to_vec};
use serde::Serialize;
use serde::de::DeserializeOwned;
use ses_core::testimony::Testimony;
use ses_core::versioned::{Lineage, Root, Versioned};

use crate::error::AdapterError;

mod decode_lineage;

pub use decode_lineage::{DecodeLineage, decode_from_predecessor};

/// Versioned SES payload with persisted testimony (Vocabulary §1.1–§1.2).
pub trait SesPayload: Serialize + DeserializeOwned + Versioned + Lineage + Testimony {}

impl<T> SesPayload for T where T: Serialize + DeserializeOwned + Versioned + Lineage + Testimony {}

/// Encode a testimony payload: byte 0 = schema version, bytes 1.. = bincode body.
pub fn encode_payload<T: SesPayload>(value: &T) -> Result<Vec<u8>, AdapterError> {
    let body = encode_to_vec(value, standard()).map_err(|e| AdapterError::Codec(e.to_string()))?;
    let mut out = Vec::with_capacity(1 + body.len());
    out.push(T::VERSION);
    out.extend(body);
    Ok(out)
}

/// Decode through the version lineage chain (Vocabulary §1.2).
pub fn decode_payload<T>(data: &[u8]) -> Result<T, AdapterError>
where
    T: SesPayload + DecodeLineage,
{
    T::decode_lineage(data)
}

pub(crate) fn decode_body<T: DeserializeOwned>(data: &[u8]) -> Result<T, AdapterError> {
    if data.is_empty() {
        return Err(AdapterError::Codec("empty payload".into()));
    }
    decode_from_slice::<T, _>(&data[1..], standard())
        .map(|(v, _)| v)
        .map_err(|e| AdapterError::Codec(e.to_string()))
}

pub(crate) fn check_version_bounds<T: Versioned>(version: u8) -> Result<(), AdapterError> {
    if version > T::VERSION {
        return Err(AdapterError::SchemaVersionMismatch {
            expected: T::VERSION,
            found: version,
        });
    }
    Ok(())
}

pub(crate) fn decode_genesis<T>(data: &[u8]) -> Result<T, AdapterError>
where
    T: Versioned<Supersedes = Root> + DeserializeOwned,
{
    if data.is_empty() {
        return Err(AdapterError::Codec("empty payload".into()));
    }
    let version = data[0];
    check_version_bounds::<T>(version)?;
    if version != T::VERSION {
        return Err(AdapterError::SchemaVersionMismatch {
            expected: T::VERSION,
            found: version,
        });
    }
    decode_body(data)
}

pub(crate) fn decode_successor<T, Prev>(data: &[u8]) -> Result<T, AdapterError>
where
    T: Versioned<Supersedes = Prev> + From<Prev> + DeserializeOwned,
    Prev: Versioned + DecodeLineage,
{
    if data.is_empty() {
        return Err(AdapterError::Codec("empty payload".into()));
    }
    let version = data[0];
    check_version_bounds::<T>(version)?;
    if version == T::VERSION {
        return decode_body(data);
    }
    let prev = Prev::decode_lineage(data)?;
    Ok(prev.into())
}
