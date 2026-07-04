//! Convention validators against adapter schema examples (ses-vocabulary §1.1).

use ses_core::convention::{
    validate_counter_name, validate_edge_kind, validate_role, validate_space_name,
};

#[test]
fn valid_edge_kinds_from_schema() {
    let kinds = [
        "project.contains",
        "element.uses_material",
        "demand.applies_to",
        "analysis.evaluates",
        "check.cites",
        "provision.supersedes",
        "code.adopts",
        "provision.modifies",
        "provision.requires",
    ];
    for kind in kinds {
        validate_edge_kind(kind).unwrap_or_else(|e| panic!("{kind}: {e}"));
    }
}

#[test]
fn invalid_edge_kinds_near_miss() {
    let bad = [
        "Project.contains",
        "project",
        "project.",
        ".contains",
        "project..contains",
        "project-contains",
    ];
    for kind in bad {
        assert!(validate_edge_kind(kind).is_err(), "expected reject: {kind}");
    }
}

#[test]
fn valid_roles_from_schema() {
    let roles = [
        "owner",
        "component",
        "consumer",
        "material",
        "demand",
        "subject",
        "analysis",
        "finding",
        "provision",
        "successor",
        "predecessor",
        "adopter",
        "adopted",
        "modifier",
        "modified",
        "dependent",
        "prerequisite",
    ];
    for role in roles {
        validate_role(role).unwrap_or_else(|e| panic!("{role}: {e}"));
    }
}

#[test]
fn invalid_roles_near_miss() {
    let bad = ["Owner", "owner-role", "", "owner role", "1owner"];
    for role in bad {
        assert!(validate_role(role).is_err(), "expected reject: {role}");
    }
}

#[test]
fn valid_counters_from_schema() {
    for name in ["proj", "edge"] {
        validate_counter_name(name).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
    for name in ["elem:1", "matl:42", "combo:1:2", "run:10:20"] {
        validate_counter_name(name).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
}

#[test]
fn invalid_counters_near_miss() {
    let bad = [
        "Proj",
        "element:1",
        "elem:",
        "elem:abc",
        "combo:1",
        "run:1:2:3",
        "edge:1",
    ];
    for name in bad {
        assert!(
            validate_counter_name(name).is_err(),
            "expected reject: {name}"
        );
    }
}

#[test]
fn valid_space_names() {
    let names = [
        "projects",
        "elements",
        "materials",
        "demands",
        "analyses",
        "check_results",
        "codes",
        "provisions",
        "relations",
        "units",
    ];
    for name in names {
        validate_space_name(name).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
}

#[test]
fn invalid_space_names_near_miss() {
    let bad = [
        "Projects",
        "check-results",
        "",
        "check results",
        "1projects",
    ];
    for name in bad {
        assert!(
            validate_space_name(name).is_err(),
            "expected reject: {name}"
        );
    }
}
