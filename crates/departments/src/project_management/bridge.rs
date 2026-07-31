use crate::project_management::payloads::ProjectPhase;
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
    /// One shot rollup for every project — avoids N+1 on the portfolio board.
    PortfolioProgress,
}

