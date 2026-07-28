//! Pod manifests — declared I/O slots for data-flow wiring.

use serde::{Deserialize, Serialize};
use ses_shell::PodKind;

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

/// Declares a pod type offered by a module, with optional I/O slots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PodManifest {
    pub kind: PodKind,
    pub display_name: String,
    pub inputs: Vec<SlotDecl>,
    pub outputs: Vec<SlotDecl>,
}

impl PodManifest {
    pub fn simple(kind: PodKind) -> Self {
        Self {
            kind,
            display_name: kind.display_name().to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn with_io(
        kind: PodKind,
        inputs: Vec<SlotDecl>,
        outputs: Vec<SlotDecl>,
    ) -> Self {
        Self {
            kind,
            display_name: kind.display_name().to_string(),
            inputs,
            outputs,
        }
    }
}
