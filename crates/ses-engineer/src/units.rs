pub mod registry;

use serde::{Deserialize, Serialize};

pub use registry::{convert_quantity, UnitRegistry};
pub use ses_core::UnitId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UnitSystem {
    #[default]
    Imperial,
    Si,
}
