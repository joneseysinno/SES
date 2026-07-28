//! Hardcoded dev user — all-access for scaffolding.

use crate::permission::Permission;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: String,
    pub display_name: String,
    pub permissions: Permission,
}

impl UserContext {
    pub fn dev_all_access() -> Self {
        Self {
            user_id: "dev".into(),
            display_name: "Developer".into(),
            permissions: Permission::ALL,
        }
    }

    pub fn can_use(&self, required: Permission) -> bool {
        self.permissions.contains(required) || self.permissions.contains(Permission::ADMIN)
    }
}
