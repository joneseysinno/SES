//! Naming law for spaces, counters, edge kinds, and roles (ses-vocabulary §1.1).

mod error;
mod validate_counter_name;
mod validate_edge_kind;
mod validate_role;
mod validate_space_name;

pub use error::ConventionError;
pub use validate_counter_name::validate_counter_name;
pub use validate_edge_kind::validate_edge_kind;
pub use validate_role::validate_role;
pub use validate_space_name::validate_space_name;
