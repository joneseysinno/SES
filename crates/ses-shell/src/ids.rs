//! Stable identifiers for shell entities.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) fn next_u64() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Reset the id counter (tests / deterministic seeding).
pub fn reset_id_counter(start: u64) {
    NEXT_ID.store(start, Ordering::Relaxed);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub u64);

impl WorkspaceId {
    pub fn new() -> Self {
        Self(next_u64())
    }
}

impl Default for WorkspaceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ws-{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LeafId(pub u64);

impl LeafId {
    pub fn new() -> Self {
        Self(next_u64())
    }
}

impl Default for LeafId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LeafId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "leaf-{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PodId(pub u64);

impl PodId {
    pub fn new() -> Self {
        Self(next_u64())
    }

    /// Fixed id for page-authored pods (must not use [`Self::new`] each render).
    pub const fn stable(id: u64) -> Self {
        Self(id)
    }
}

impl Default for PodId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pod-{}", self.0)
    }
}

/// Module identifier — string key for registry lookup.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub String);

impl ModuleId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for ModuleId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
