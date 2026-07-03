use infinite_db::infinitedb_core::kind_catalog::{
    DirectionalityPolicy, KindCatalog, KindDefinition, UnknownKindPolicy,
};

use crate::schema::edges::{ALL_EDGE_KINDS, ALL_ENDPOINT_ROLES};

/// Build the SES hyperedge kind catalog with `RejectUnknown` policy.
pub fn build_kind_catalog() -> KindCatalog {
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
