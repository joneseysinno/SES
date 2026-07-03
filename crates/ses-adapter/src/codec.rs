use bincode::config::standard;
use bincode::serde::{decode_from_slice, encode_to_vec};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::AdapterError;

/// Versioned SES payload: byte 0 = schema version, bytes 1.. = bincode payload.
pub trait SesPayload: Serialize + DeserializeOwned {
    const SCHEMA_VERSION: u8;
}

pub fn encode_payload<T: SesPayload>(value: &T) -> Result<Vec<u8>, AdapterError> {
    let body = encode_to_vec(value, standard()).map_err(|e| AdapterError::Codec(e.to_string()))?;
    let mut out = Vec::with_capacity(1 + body.len());
    out.push(T::SCHEMA_VERSION);
    out.extend(body);
    Ok(out)
}

pub fn decode_payload<T: SesPayload>(data: &[u8]) -> Result<T, AdapterError> {
    if data.is_empty() {
        return Err(AdapterError::Codec("empty payload".into()));
    }
    let version = data[0];
    if version != T::SCHEMA_VERSION {
        return Err(AdapterError::SchemaVersionMismatch {
            expected: T::SCHEMA_VERSION,
            found: version,
        });
    }
    decode_from_slice::<T, _>(&data[1..], standard())
        .map(|(v, _)| v)
        .map_err(|e| AdapterError::Codec(e.to_string()))
}
