use crate::shared::{Address, Client, Money, ProjectId};
use serde::{Deserialize, Serialize};
use ses_adapter::payload::{DesignBasis, ProjectStatus};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

/// Portfolio Kanban columns — the project lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectPhase {
    Prospect,
    Proposal,
    Awarded,
    InProgress,
    UnderReview,
    Delivered,
    Closed,
}

impl ProjectPhase {
    pub fn all() -> &'static [ProjectPhase] {
        &[
            ProjectPhase::Prospect,
            ProjectPhase::Proposal,
            ProjectPhase::Awarded,
            ProjectPhase::InProgress,
            ProjectPhase::UnderReview,
            ProjectPhase::Delivered,
            ProjectPhase::Closed,
        ]
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Prospect => "Prospect",
            Self::Proposal => "Proposal",
            Self::Awarded => "Awarded",
            Self::InProgress => "In Progress",
            Self::UnderReview => "Under Review",
            Self::Delivered => "Delivered",
            Self::Closed => "Closed",
        }
    }

    pub fn accent(self) -> &'static str {
        match self {
            Self::Prospect => "phase-prospect",
            Self::Proposal => "phase-proposal",
            Self::Awarded => "phase-awarded",
            Self::InProgress => "phase-active",
            Self::UnderReview => "phase-review",
            Self::Delivered => "phase-delivered",
            Self::Closed => "phase-closed",
        }
    }

    pub fn column_id(self) -> &'static str {
        match self {
            Self::Prospect => "prospect",
            Self::Proposal => "proposal",
            Self::Awarded => "awarded",
            Self::InProgress => "in-progress",
            Self::UnderReview => "under-review",
            Self::Delivered => "delivered",
            Self::Closed => "closed",
        }
    }

    pub fn from_status(status: ProjectStatus) -> Self {
        match status {
            ProjectStatus::Draft => Self::Prospect,
            ProjectStatus::Active => Self::InProgress,
            ProjectStatus::Issued => Self::Delivered,
            ProjectStatus::Archived => Self::Closed,
        }
    }
}

/// The project record — management fields and engineering design basis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectRecord {
    pub id: ProjectId,
    pub name: String,
    pub number: String,
    pub client: Client,
    pub address: Address,
    pub status: ProjectStatus,
    pub phase: ProjectPhase,
    pub manager: String,
    pub start_utc: i64,
    pub target_finish_utc: Option<i64>,
    pub contract_value: Option<Money>,
    pub design_basis: DesignBasis,
    pub engineer_of_record: String,
    pub created_utc: i64,
}

impl Versioned for ProjectRecord {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for ProjectRecord {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["project manager", "engineer"];
}
