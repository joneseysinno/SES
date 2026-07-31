use crate::project_management::payloads::ProjectPhase;
use crate::shared::ProjectId;
use ses_adapter::payload::ProjectStatus;

#[derive(Debug, Clone, Default)]
pub struct NewProjectParams {
    pub name: String,
    pub number: String,
    pub client: Option<String>,
    pub manager: Option<String>,
    pub target_finish_utc: Option<i64>,
    pub contract_value_cents: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectPatch {
    pub name: Option<String>,
}

/// Constraints for `ListAll`. Empty vectors mean "no constraint" — an
/// all-`Default` filter matches every project.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProjectFilter {
    /// Empty ⇒ no constraint.
    pub statuses: Vec<ProjectStatus>,
    pub phases: Vec<ProjectPhase>,
    /// Case-insensitive substring across number, name, and client.
    pub text: Option<String>,
}

impl ProjectFilter {
    pub fn active() -> Self {
        Self {
            statuses: vec![ProjectStatus::Active],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewProposalParams {
    pub project_id: ProjectId,
    pub scope: String,
    pub fee_cents: Option<i64>,
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
    /// One shot rollup for every project — avoids N+1 on the portfolio board.
    PortfolioProgress,
}
