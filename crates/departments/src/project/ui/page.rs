pub mod project_analysis;
pub mod project_docs;
pub mod project_overview;
pub mod project_timeline;
pub mod task_board;
pub mod task_detail;
pub mod time_tracking;

pub use project_analysis::ProjectAnalysisPage;
pub use project_docs::ProjectDocsPage;
pub use project_overview::ProjectOverviewPage;
pub use project_timeline::ProjectTimelinePage;
pub use task_board::TaskBoardPage;
pub use task_detail::TaskDetailPage;
pub use time_tracking::TimeTrackingPage;

/// Stable page id strings. These are persisted inside `PageDescriptor`
/// in saved workspaces — treat them as a data contract, not a label.
pub const TASK_BOARD: &str = "task-board";
pub const TASK_DETAIL: &str = "task-detail";
pub const PROJECT_DOCS: &str = "project-docs";
pub const PROJECT_ANALYSIS: &str = "project-analysis";
pub const TIME_TRACKING: &str = "time-tracking";
pub const PROJECT_TIMELINE: &str = "project-timeline";
pub const PROJECT_OVERVIEW: &str = "project-overview";

/// Every page this department offers. `module.rs` builds one manifest per
/// entry; `ui.rs` dispatches one arm per entry. Adding a page means adding
/// it here first.
pub const ALL: &[&str] = &[
    TASK_BOARD,
    TASK_DETAIL,
    PROJECT_DOCS,
    PROJECT_ANALYSIS,
    TIME_TRACKING,
    PROJECT_TIMELINE,
    PROJECT_OVERVIEW,
];

/// Exhaustive page enum for dispatch. Built from the same roster as [`ALL`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    TaskBoard,
    TaskDetail,
    ProjectDocs,
    ProjectAnalysis,
    TimeTracking,
    ProjectTimeline,
    ProjectOverview,
}

impl Page {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            TASK_BOARD => Some(Self::TaskBoard),
            TASK_DETAIL => Some(Self::TaskDetail),
            PROJECT_DOCS => Some(Self::ProjectDocs),
            PROJECT_ANALYSIS => Some(Self::ProjectAnalysis),
            TIME_TRACKING => Some(Self::TimeTracking),
            PROJECT_TIMELINE => Some(Self::ProjectTimeline),
            PROJECT_OVERVIEW => Some(Self::ProjectOverview),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::TaskBoard => TASK_BOARD,
            Self::TaskDetail => TASK_DETAIL,
            Self::ProjectDocs => PROJECT_DOCS,
            Self::ProjectAnalysis => PROJECT_ANALYSIS,
            Self::TimeTracking => TIME_TRACKING,
            Self::ProjectTimeline => PROJECT_TIMELINE,
            Self::ProjectOverview => PROJECT_OVERVIEW,
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
        assert_eq!(Page::from_id("not-a-page"), None);
    }
}
