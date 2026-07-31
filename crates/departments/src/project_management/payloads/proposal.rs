use crate::shared::{Money, ProjectId, ProposalId};
use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

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

impl Versioned for Proposal {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for Proposal {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["project manager"];
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
