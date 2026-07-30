use serde::{Deserialize, Serialize};
use ses_adapter::AdapterError;
use ses_adapter::codec::{DecodeLineage, decode_from_predecessor, decode_payload, encode_payload};
use departments::project_management::ProjectRecord;
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{FromPrev, Genesis, Root, Versioned};

fn sample_design_basis() -> ses_adapter::payload::DesignBasis {
    ses_adapter::payload::DesignBasis {
        code_stack: Vec::new(),
        amendment_branch: None,
        display_units: ses_adapter::payload::UnitSystemPref::Imperial,
        sds_milli: 0,
        sd1_milli: 0,
        seismic_design_category: String::new(),
        risk_category: ses_adapter::payload::RiskCategory::Ii,
    }
}

fn sample_project_record() -> ProjectRecord {
    use departments::project_management::ProjectPhase;
    use departments::shared::{Address, Client, ProjectId};

    ProjectRecord {
        id: ProjectId::from_raw(1),
        name: "Clinic Addition".into(),
        number: "2026-001".into(),
        client: Client::from_name("Example Client"),
        address: Address::from_freeform("Salt Lake City, UT"),
        status: ses_adapter::payload::ProjectStatus::Draft,
        phase: ProjectPhase::Prospect,
        manager: String::new(),
        start_utc: 1_700_000_000,
        target_finish_utc: None,
        contract_value: None,
        design_basis: sample_design_basis(),
        engineer_of_record: "Dana, PE".into(),
        created_utc: 1_700_000_000,
    }
}

#[test]
fn project_record_round_trip() {
    let record = sample_project_record();

    let encoded = encode_payload(&record).expect("encode");
    assert_eq!(encoded[0], ProjectRecord::VERSION);

    let decoded: ProjectRecord = decode_payload(&encoded).expect("decode");
    assert_eq!(decoded.name, record.name);
    assert_eq!(decoded.number, record.number);
    assert_eq!(decoded.engineer_of_record, record.engineer_of_record);
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

#[test]
fn schema_names_pass_naming_law() {
    ses_adapter::catalog::validate_schema_names().expect("schema names valid");
}

// --- mock lineage chain (Vocabulary §1.2) ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MockPayloadV1 {
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MockPayloadV2 {
    label: String,
    revision: u8,
}

impl Versioned for MockPayloadV1 {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Versioned for MockPayloadV2 {
    const VERSION: u8 = 2;
    type Supersedes = MockPayloadV1;
    type LineageVia = FromPrev<MockPayloadV1>;
}

impl From<MockPayloadV1> for MockPayloadV2 {
    fn from(v1: MockPayloadV1) -> Self {
        Self {
            label: v1.label,
            revision: 0,
        }
    }
}

impl Testimony for MockPayloadV1 {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}

impl Testimony for MockPayloadV2 {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["engineer"];
}

impl DecodeLineage for MockPayloadV2 {
    fn decode_lineage(data: &[u8]) -> Result<Self, AdapterError> {
        decode_from_predecessor::<Self, MockPayloadV1>(data)
    }
}

#[test]
fn lineage_decode_v1_bytes_as_v2() {
    let v1 = MockPayloadV1 {
        label: "legacy".into(),
    };
    let encoded = encode_payload(&v1).expect("encode v1");
    assert_eq!(encoded[0], 1);

    let decoded: MockPayloadV2 = decode_payload(&encoded).expect("decode as v2");
    assert_eq!(decoded.label, "legacy");
    assert_eq!(decoded.revision, 0);
}

#[test]
fn lineage_rejects_future_version() {
    let v2 = MockPayloadV2 {
        label: "current".into(),
        revision: 3,
    };
    let encoded = encode_payload(&v2).expect("encode v2");

    let err = decode_payload::<MockPayloadV1>(&encoded).unwrap_err();
    assert!(matches!(
        err,
        ses_adapter::AdapterError::SchemaVersionMismatch { .. }
    ));
}
