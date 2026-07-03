pub mod registry;

use serde::{Deserialize, Serialize};

pub use registry::{convert_quantity, UnitRegistry};

/// Opaque identifier for a row in the units registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct UnitId(pub u32);

/// Dimension signature F^a · L^b · T^c · Θ^d.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct DimensionSignature {
    pub force: i8,
    pub length: i8,
    pub time: i8,
    pub temperature: i8,
}

impl DimensionSignature {
    pub const fn dimensionless() -> Self {
        Self {
            force: 0,
            length: 0,
            time: 0,
            temperature: 0,
        }
    }

    pub const fn length() -> Self {
        Self {
            force: 0,
            length: 1,
            time: 0,
            temperature: 0,
        }
    }

    pub const fn force() -> Self {
        Self {
            force: 1,
            length: 0,
            time: 0,
            temperature: 0,
        }
    }

    pub const fn stress() -> Self {
        Self {
            force: 1,
            length: -2,
            time: 0,
            temperature: 0,
        }
    }

    pub const fn moment() -> Self {
        Self {
            force: 1,
            length: 1,
            time: 0,
            temperature: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UnitSystem {
    #[default]
    Imperial,
    Si,
}
