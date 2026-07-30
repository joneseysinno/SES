use crate::project_management::payloads::{ProjectPhase, ProjectRecord};
use crate::shared::ProjectId;

#[derive(Debug, Clone, Default)]
pub struct NewProjectParams {
    pub name: String,
    pub number: String,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectPatch {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectFilter;

#[derive(Debug, Clone)]
pub struct NewProposalParams {
    pub project_id: ProjectId,
    pub scope: String,
}

#[derive(Debug, Clone, Default)]
pub struct ProposalPatch {
    pub scope: Option<String>,
}

pub enum ProjectMgmtCommand {
    CreateProject(NewProjectParams),
    UpdateProject { id: ProjectId, patch: ProjectPatch },
    SetPhase { id: ProjectId, phase: ProjectPhase },
    SetStatus {
        id: ProjectId,
        status: ses_adapter::payload::ProjectStatus,
    },
    ArchiveProject { id: ProjectId },
    CreateProposal(NewProposalParams),
    ReviseProposal { id: crate::shared::ProposalId, patch: ProposalPatch },
    SendProposal { id: crate::shared::ProposalId },
    OpenProjectWorkspace { project_id: ProjectId },
}

pub enum ProjectMgmtQuery {
    ListAll { filter: ProjectFilter },
    GetProject(ProjectId),
    ListProposals(ProjectId),
    PortfolioMetrics,
}

/// Mock portfolio data for scaffold pages.
pub fn mock_projects() -> Vec<ProjectRecord> {
    use crate::shared::{Address, Client, Money};
    use ses_adapter::payload::{
        DesignBasis, ProjectStatus, RiskCategory, UnitSystemPref,
    };

    vec![ProjectRecord {
        id: ProjectId::from_raw(1),
        name: "Clinic Addition".into(),
        number: "2026-001".into(),
        client: Client::from_name("Example Health"),
        address: Address::from_freeform("Salt Lake City, UT"),
        status: ProjectStatus::Active,
        phase: ProjectPhase::InProgress,
        manager: "Alex PM".into(),
        start_utc: 1_700_000_000,
        target_finish_utc: Some(1_800_000_000),
        contract_value: Some(Money::usd(250_000_00)),
        design_basis: DesignBasis {
            code_stack: vec![],
            amendment_branch: None,
            display_units: UnitSystemPref::Imperial,
            sds_milli: 0,
            sd1_milli: 0,
            seismic_design_category: "D".into(),
            risk_category: RiskCategory::Ii,
        },
        engineer_of_record: "Dana, PE".into(),
        created_utc: 1_700_000_000,
    }]
}
