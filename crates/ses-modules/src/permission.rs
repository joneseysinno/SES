//! Permission bitflags for module capabilities.

use serde::{Deserialize, Serialize};

/// Bitflags for module-level capabilities (scaffolding — expand later).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission(pub u64);

impl Permission {
    pub const NONE: Permission = Permission(0);
    pub const VIEW: Permission = Permission(1 << 0);
    pub const EDIT: Permission = Permission(1 << 1);
    pub const ANALYZE: Permission = Permission(1 << 2);
    pub const ADMIN: Permission = Permission(1 << 3);
    pub const ALL: Permission = Permission(u64::MAX);

    pub fn contains(self, other: Permission) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: Permission) -> Permission {
        Permission(self.0 | other.0)
    }

    pub fn intersects(self, other: Permission) -> bool {
        self.0 & other.0 != 0
    }
}

impl std::ops::BitOr for Permission {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}
