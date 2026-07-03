use serde::{Deserialize, Serialize};
use ses_engineer::Quantity;

use crate::codec::SesPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialKind {
    Concrete {
        fc: Quantity,
        lambda: Quantity,
        wc: Quantity,
    },
    Rebar {
        fy: Quantity,
        fu: Quantity,
        grade: String,
    },
}

impl Default for MaterialKind {
    fn default() -> Self {
        let q = Quantity::new(
            ses_engineer::Rational::one(),
            ses_engineer::UnitId(0),
            "",
        );
        Self::Concrete {
            fc: q.clone(),
            lambda: q.clone(),
            wc: q,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Material {
    pub kind: MaterialKind,
    pub label: String,
}

impl SesPayload for Material {
    const SCHEMA_VERSION: u8 = 1;
}
