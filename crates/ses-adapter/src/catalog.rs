use infinite_db::infinitedb_core::kind_catalog::{
    DirectionalityPolicy, KindCatalog, KindDefinition, UnknownKindPolicy,
};
use ses_core::{
    ConventionError, validate_counter_name, validate_edge_kind, validate_role, validate_space_name,
};

use crate::schema::counters::{COUNTER_EDGE, COUNTER_PROJ};
use crate::schema::edges::{ALL_EDGE_KINDS, ALL_ENDPOINT_ROLES};
use crate::schema::spaces::ALL_SPACE_NAMES;

/// Build the SES hyperedge kind catalog with `RejectUnknown` policy.
///
/// Naming-law validators run at registration time (ses-vocabulary §4–§5).
pub fn build_kind_catalog() -> KindCatalog {
    validate_schema_names().expect("SES schema names violate naming law");

    let mut catalog = KindCatalog::new(UnknownKindPolicy::RejectUnknown);

    for kind in ALL_EDGE_KINDS {
        catalog.register_edge_kind(
            KindDefinition::new(*kind).with_directionality(DirectionalityPolicy::ObligateDirected),
        );
    }

    for role in ALL_ENDPOINT_ROLES {
        catalog.register_endpoint_role(KindDefinition::new(*role));
    }

    catalog
}

/// Validate edge kinds, endpoint roles, counters, and space names before registration.
pub fn validate_schema_names() -> Result<(), ConventionError> {
    for kind in ALL_EDGE_KINDS {
        validate_edge_kind(kind)?;
    }
    for role in ALL_ENDPOINT_ROLES {
        validate_role(role)?;
    }
    for name in ALL_SPACE_NAMES {
        validate_space_name(name)?;
    }
    validate_counter_name(COUNTER_PROJ)?;
    validate_counter_name(COUNTER_EDGE)?;
    Ok(())
}
