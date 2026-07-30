/// Hyperedge kind labels (ses-vocabulary §5 + ses-code-pipeline §6.1).
pub const KIND_PROJECT_CONTAINS: &str = "project.contains";
pub const KIND_ELEMENT_USES_MATERIAL: &str = "element.uses_material";
pub const KIND_DEMAND_APPLIES_TO: &str = "demand.applies_to";
pub const KIND_ANALYSIS_EVALUATES: &str = "analysis.evaluates";
pub const KIND_CHECK_CITES: &str = "check.cites";
pub const KIND_PROVISION_SUPERSEDES: &str = "provision.supersedes";
pub const KIND_CODE_ADOPTS: &str = "code.adopts";
pub const KIND_PROVISION_MODIFIES: &str = "provision.modifies";
pub const KIND_PROVISION_REQUIRES: &str = "provision.requires";

/// Endpoint role labels.
pub const ROLE_OWNER: &str = "owner";
pub const ROLE_COMPONENT: &str = "component";
pub const ROLE_CONSUMER: &str = "consumer";
pub const ROLE_MATERIAL: &str = "material";
pub const ROLE_DEMAND: &str = "demand";
pub const ROLE_SUBJECT: &str = "subject";
pub const ROLE_ANALYSIS: &str = "analysis";
pub const ROLE_FINDING: &str = "finding";
pub const ROLE_PROVISION: &str = "provision";
pub const ROLE_SUCCESSOR: &str = "successor";
pub const ROLE_PREDECESSOR: &str = "predecessor";
pub const ROLE_ADOPTER: &str = "adopter";
pub const ROLE_ADOPTED: &str = "adopted";
pub const ROLE_MODIFIER: &str = "modifier";
pub const ROLE_MODIFIED: &str = "modified";
pub const ROLE_DEPENDENT: &str = "dependent";
pub const ROLE_PREREQUISITE: &str = "prerequisite";

pub const ALL_EDGE_KINDS: &[&str] = &[
    KIND_PROJECT_CONTAINS,
    KIND_ELEMENT_USES_MATERIAL,
    KIND_DEMAND_APPLIES_TO,
    KIND_ANALYSIS_EVALUATES,
    KIND_CHECK_CITES,
    KIND_PROVISION_SUPERSEDES,
    KIND_CODE_ADOPTS,
    KIND_PROVISION_MODIFIES,
    KIND_PROVISION_REQUIRES,
];

pub const ALL_ENDPOINT_ROLES: &[&str] = &[
    ROLE_OWNER,
    ROLE_COMPONENT,
    ROLE_CONSUMER,
    ROLE_MATERIAL,
    ROLE_DEMAND,
    ROLE_SUBJECT,
    ROLE_ANALYSIS,
    ROLE_FINDING,
    ROLE_PROVISION,
    ROLE_SUCCESSOR,
    ROLE_PREDECESSOR,
    ROLE_ADOPTER,
    ROLE_ADOPTED,
    ROLE_MODIFIER,
    ROLE_MODIFIED,
    ROLE_DEPENDENT,
    ROLE_PREREQUISITE,
];
