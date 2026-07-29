//! Shared field metadata for centralized IO components.

/// Common display / validation metadata for an IO field.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FieldMeta {
    pub id: String,
    pub label: String,
    pub units: Option<String>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub error: Option<String>,
}

impl FieldMeta {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn with_units(mut self, units: impl Into<String>) -> Self {
        self.units = Some(units.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Extra documentation shown when an engineer input info panel is expanded.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EngineerInfo {
    pub description: Option<String>,
    pub code_refs: Vec<String>,
    pub validation_notes: Vec<String>,
}

impl EngineerInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_code_ref(mut self, code_ref: impl Into<String>) -> Self {
        self.code_refs.push(code_ref.into());
        self
    }

    pub fn with_validation_note(mut self, note: impl Into<String>) -> Self {
        self.validation_notes.push(note.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.description.as_ref().is_none_or(|s| s.is_empty())
            && self.code_refs.is_empty()
            && self.validation_notes.is_empty()
    }
}
