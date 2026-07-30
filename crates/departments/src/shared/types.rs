//! Cross-department domain nouns.

use serde::{Deserialize, Serialize};

/// Client identity and contact details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub contact: ContactInfo,
}

impl Client {
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contact: ContactInfo::default(),
        }
    }
}

/// Contact channels for a client.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: String,
    pub phone: String,
}

/// Postal address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal: String,
}

impl Address {
    pub fn from_freeform(text: impl Into<String>) -> Self {
        Self {
            street: text.into(),
            city: String::new(),
            state: String::new(),
            postal: String::new(),
        }
    }
}

/// Exact money — integer cents (NoFloats policy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub cents: i64,
    pub currency: Currency,
}

impl Money {
    pub fn usd(cents: i64) -> Self {
        Self {
            cents,
            currency: Currency::Usd,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Currency {
    #[default]
    Usd,
}

/// Exact duration — integer minutes, never float hours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Minutes(pub u32);

impl Minutes {
    pub fn from_hours(h: u32) -> Self {
        Self(h.saturating_mul(60))
    }

    /// Display only — never used in arithmetic.
    pub fn as_hours_display(self) -> String {
        format!("{}h {:02}m", self.0 / 60, self.0 % 60)
    }
}

/// Inclusive UTC date range (seconds since epoch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    pub start_utc: i64,
    pub end_utc: i64,
}
