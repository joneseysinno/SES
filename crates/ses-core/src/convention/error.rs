/// Naming-law violation (ses-vocabulary §1.1, §4–§5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConventionError {
    /// Hyperedge kind label failed validation.
    InvalidEdgeKind {
        /// Offending label.
        label: String,
        /// Human-readable reason.
        reason: &'static str,
    },
    /// Endpoint role label failed validation.
    InvalidRole {
        /// Offending label.
        label: String,
        /// Human-readable reason.
        reason: &'static str,
    },
    /// Counter name failed validation.
    InvalidCounterName {
        /// Offending name.
        name: String,
        /// Human-readable reason.
        reason: &'static str,
    },
    /// Space registration name failed validation.
    InvalidSpaceName {
        /// Offending name.
        name: String,
        /// Human-readable reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for ConventionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidEdgeKind { label, reason } => {
                write!(f, "invalid edge kind `{label}`: {reason}")
            }
            Self::InvalidRole { label, reason } => {
                write!(f, "invalid role `{label}`: {reason}")
            }
            Self::InvalidCounterName { name, reason } => {
                write!(f, "invalid counter name `{name}`: {reason}")
            }
            Self::InvalidSpaceName { name, reason } => {
                write!(f, "invalid space name `{name}`: {reason}")
            }
        }
    }
}

impl core::error::Error for ConventionError {}
