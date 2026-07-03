use serde::{Deserialize, Serialize};

/// Declared adele-ring tower level for a provision expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TowerLevel {
    #[default]
    Rational,
    Algebraic,
    Computable,
}
