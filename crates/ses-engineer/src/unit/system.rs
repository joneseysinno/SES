use serde::{Deserialize, Serialize};

/// Unit measurement system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UnitSystem {
    #[default]
    Imperial,
    Si,
}
