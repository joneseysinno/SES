use crate::shared::{Money, ProjectId, ProposalId};
use serde::{Deserialize, Serialize};

/// Proposal authored against a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    pub id: ProposalId,
    pub project_id: ProjectId,
    pub revision: u16,
    pub scope: String,
    pub fee: Money,
    pub schedule_weeks: u16,
    pub status: ProposalStatus,
    pub sent_utc: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProposalStatus {
    #[default]
    Draft,
    Sent,
    Accepted,
    Declined,
    Superseded,
}
