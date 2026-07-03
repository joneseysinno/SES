use ses_adapter::codec::{decode_payload, encode_payload, SesPayload};
use ses_adapter::payload::Project;

#[test]
fn project_round_trip() {
    let project = Project {
        name: "Clinic Addition".into(),
        project_number: "2026-001".into(),
        client: "Example Client".into(),
        address: "Salt Lake City, UT".into(),
        design_basis: Default::default(),
        engineer_of_record: "Dana, PE".into(),
        status: Default::default(),
        created_utc: 1_700_000_000,
    };

    let encoded = encode_payload(&project).expect("encode");
    assert_eq!(encoded[0], Project::SCHEMA_VERSION);

    let decoded: Project = decode_payload(&encoded).expect("decode");
    assert_eq!(decoded.name, project.name);
    assert_eq!(decoded.project_number, project.project_number);
}

#[test]
fn kind_catalog_registers_all_edge_kinds() {
    let catalog = ses_adapter::build_kind_catalog();
    catalog
        .validate_edge_kind(ses_adapter::schema::KIND_ANALYSIS_EVALUATES)
        .expect("analysis.evaluates registered");
    catalog
        .validate_endpoint_role(ses_adapter::schema::ROLE_SUBJECT)
        .expect("subject role registered");
}
