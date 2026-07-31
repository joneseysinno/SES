pub mod projects_overview;
pub mod timecard;

pub use projects_overview::ProjectsOverviewPage;
pub use timecard::TimecardPage;

/// Stable page id strings. These are persisted inside `PageDescriptor`
/// in saved workspaces — treat them as a data contract, not a label.
pub const PROJECTS_OVERVIEW: &str = "projects-overview";
pub const TIMECARD: &str = "timecard";

/// Every page this department offers. `module.rs` builds one manifest per
/// entry; `ui.rs` dispatches one arm per entry. Adding a page means adding
/// it here first.
pub const ALL: &[&str] = &[PROJECTS_OVERVIEW, TIMECARD];

/// Exhaustive page enum for dispatch. Built from the same roster as [`ALL`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    ProjectsOverview,
    Timecard,
}

impl Page {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            PROJECTS_OVERVIEW => Some(Self::ProjectsOverview),
            TIMECARD => Some(Self::Timecard),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::ProjectsOverview => PROJECTS_OVERVIEW,
            Self::Timecard => TIMECARD,
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
