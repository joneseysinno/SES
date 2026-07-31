//! Cross-department identifier newtypes.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_u64() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Reset the id counter (tests / deterministic seeding).
pub fn reset_id_counter(start: u64) {
    NEXT_ID.store(start, Ordering::Relaxed);
}

macro_rules! id_type {
    ($name:ident, $prefix:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $name(pub u64);

        impl $name {
            pub fn new() -> Self {
                Self(next_u64())
            }

            pub fn from_raw(raw: u64) -> Self {
                Self(raw)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($prefix, "-{}"), self.0)
            }
        }
    };
}

id_type!(ProjectId, "proj");
id_type!(TaskId, "task");
id_type!(BoardCardId, "card");
id_type!(MilestoneId, "ms");
id_type!(TimeEntryId, "time");
id_type!(DocRefId, "doc");
id_type!(ProposalId, "prop");
