//! Data-flow bus — pub/sub between pod I/O containers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Channel key used by scaffolding bindings (string for simplicity).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FlowChannelId(pub String);

impl FlowChannelId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for FlowChannelId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for FlowChannelId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for FlowChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Named slot on a pod (input or output).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FlowSlot {
    pub name: String,
}

impl FlowSlot {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Placeholder values until ses-engineer quantities are wired.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlowValue {
    /// f64 placeholder — replace with exact rational later.
    Number(f64),
    Text(String),
    Bool(bool),
    Json(serde_json::Value),
}

impl FlowValue {
    pub fn display(&self) -> String {
        match self {
            Self::Number(n) => format!("{n}"),
            Self::Text(s) => s.clone(),
            Self::Bool(b) => b.to_string(),
            Self::Json(v) => v.to_string(),
        }
    }
}

impl fmt::Display for FlowValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display())
    }
}

/// In-memory pub/sub bus keyed by channel id.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowBus {
    channels: HashMap<FlowChannelId, FlowValue>,
}

impl FlowBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&mut self, channel: impl Into<FlowChannelId>, value: FlowValue) {
        self.channels.insert(channel.into(), value);
    }

    pub fn get(&self, channel: &FlowChannelId) -> Option<&FlowValue> {
        self.channels.get(channel)
    }

    pub fn get_str(&self, channel: &str) -> Option<&FlowValue> {
        self.channels.get(&FlowChannelId::new(channel))
    }

    pub fn clear(&mut self, channel: &FlowChannelId) {
        self.channels.remove(channel);
    }

    pub fn channels(&self) -> &HashMap<FlowChannelId, FlowValue> {
        &self.channels
    }
}
