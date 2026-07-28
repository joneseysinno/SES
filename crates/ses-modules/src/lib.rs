//! SES module registry and permission scaffolding.

pub mod permission;
pub mod registry;
pub mod slots;
pub mod stubs;
pub mod user;

pub use permission::Permission;
pub use registry::{ModuleRegistry, SesModule};
pub use slots::PodManifest;
pub use stubs::default_modules;
pub use user::UserContext;
