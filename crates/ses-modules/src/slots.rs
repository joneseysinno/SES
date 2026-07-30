//! Pod manifests — declared I/O slots for data-flow wiring (chrome / legacy).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotDecl {
    pub name: String,
    pub label: String,
}

impl SlotDecl {
    pub fn new(name: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}

/// Declares a chrome or legacy pod type offered by a module, with optional I/O slots.
/// Content pages use [`crate::PageManifest`] instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PodManifest {
    pub id: String,
    pub display_name: String,
    pub inputs: Vec<SlotDecl>,
    pub outputs: Vec<SlotDecl>,
}

impl PodManifest {
    pub fn simple(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn with_io(
        id: impl Into<String>,
        display_name: impl Into<String>,
        inputs: Vec<SlotDecl>,
        outputs: Vec<SlotDecl>,
    ) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            inputs,
            outputs,
        }
    }
}
