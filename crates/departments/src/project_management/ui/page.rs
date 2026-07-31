pub mod project_board;
pub mod project_list;
pub mod project_summary;
pub mod proposal_editor;
pub mod project_metrics;

pub use project_board::ProjectBoardPage;
pub use project_list::ProjectListPage;
pub use project_summary::ProjectSummaryPage;
pub use proposal_editor::ProposalEditorPage;
pub use project_metrics::ProjectMetricsPage;

/// Stable page id strings. These are persisted inside `PageDescriptor`
/// in saved workspaces — treat them as a data contract, not a label.
pub const PROJECT_BOARD: &str = "project-board";
pub const PROJECT_LIST: &str = "project-list";
pub const PROJECT_SUMMARY: &str = "project-summary";
pub const PROPOSAL_EDITOR: &str = "proposal-editor";
pub const PROJECT_METRICS: &str = "project-metrics";

/// Every page this department offers. `module.rs` builds one manifest per
/// entry; `ui.rs` dispatches one arm per entry. Adding a page means adding
/// it here first.
pub const ALL: &[&str] = &[
    PROJECT_BOARD,
    PROJECT_LIST,
    PROJECT_SUMMARY,
    PROPOSAL_EDITOR,
    PROJECT_METRICS,
];

/// Exhaustive page enum for dispatch. Built from the same roster as [`ALL`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    ProjectBoard,
    ProjectList,
    ProjectSummary,
    ProposalEditor,
    ProjectMetrics,
}

impl Page {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            PROJECT_BOARD => Some(Self::ProjectBoard),
            PROJECT_LIST => Some(Self::ProjectList),
            PROJECT_SUMMARY => Some(Self::ProjectSummary),
            PROPOSAL_EDITOR => Some(Self::ProposalEditor),
            PROJECT_METRICS => Some(Self::ProjectMetrics),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::ProjectBoard => PROJECT_BOARD,
            Self::ProjectList => PROJECT_LIST,
            Self::ProjectSummary => PROJECT_SUMMARY,
            Self::ProposalEditor => PROPOSAL_EDITOR,
            Self::ProjectMetrics => PROJECT_METRICS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_has_no_duplicates() {
        let mut seen = std::collections::HashSet::new();
        for id in ALL {
            assert!(seen.insert(*id), "duplicate page id in ALL: {id}");
        }
    }

    #[test]
    fn from_id_covers_all_and_rejects_unknown() {
        for id in ALL {
            assert_eq!(Page::from_id(id).map(|p| p.id()), Some(*id));
        }
        assert_eq!(Page::from_id("portfolio-board"), None);
        assert_eq!(Page::from_id("not-a-page"), None);
    }
}
