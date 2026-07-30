use crate::shared::ProjectId;
use serde::{Deserialize, Serialize};
use ses_core::testimony::{Testimony, TestimonyKind};
use ses_core::versioned::{Genesis, Root, Versioned};

use super::task::Task;

/// Identifies one Kanban column within a project's board.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColumnId(pub String);

impl ColumnId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl From<&str> for ColumnId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// One Kanban column. The two flags are what the rollup reads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnDef {
    pub id: ColumnId,
    pub title: String,
    /// Tasks here count toward the completed numerator.
    pub counts_complete: bool,
    /// Tasks here are flagged as exceptions (blocked / on hold).
    pub is_exception: bool,
    pub accent: Option<String>,
    /// WIP limit — advisory only.
    pub limit: Option<u16>,
    pub order: u16,
}

/// The board layout for one project. Authored testimony.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardConfig {
    pub project_id: ProjectId,
    pub columns: Vec<ColumnDef>,
}

impl Versioned for BoardConfig {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Testimony for BoardConfig {
    const KIND: TestimonyKind = TestimonyKind::Authored;
    const WITNESSES: &'static [&'static str] = &["project manager"];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardConfigError {
    NoColumns,
    NoCompleteColumn,
    DuplicateColumnId(ColumnId),
    CompleteAndException(ColumnId),
    ColumnHasTasks { id: ColumnId, count: u32 },
}

impl BoardConfig {
    /// Seed board for new projects.
    pub fn factory(project_id: ProjectId) -> Self {
        Self {
            project_id,
            columns: vec![
                ColumnDef {
                    id: ColumnId::new("backlog"),
                    title: "Backlog".into(),
                    counts_complete: false,
                    is_exception: false,
                    accent: None,
                    limit: None,
                    order: 0,
                },
                ColumnDef {
                    id: ColumnId::new("ready"),
                    title: "Ready".into(),
                    counts_complete: false,
                    is_exception: false,
                    accent: None,
                    limit: None,
                    order: 1,
                },
                ColumnDef {
                    id: ColumnId::new("in-progress"),
                    title: "In Progress".into(),
                    counts_complete: false,
                    is_exception: false,
                    accent: None,
                    limit: None,
                    order: 2,
                },
                ColumnDef {
                    id: ColumnId::new("blocked"),
                    title: "Blocked".into(),
                    counts_complete: false,
                    is_exception: true,
                    accent: Some("warn".into()),
                    limit: None,
                    order: 3,
                },
                ColumnDef {
                    id: ColumnId::new("review"),
                    title: "Review".into(),
                    counts_complete: false,
                    is_exception: false,
                    accent: None,
                    limit: None,
                    order: 4,
                },
                ColumnDef {
                    id: ColumnId::new("done"),
                    title: "Done".into(),
                    counts_complete: true,
                    is_exception: false,
                    accent: Some("good".into()),
                    limit: None,
                    order: 5,
                },
            ],
        }
    }

    /// Enforced on every write. NoSilentDefaults.
    pub fn validate(&self) -> Result<(), BoardConfigError> {
        if self.columns.is_empty() {
            return Err(BoardConfigError::NoColumns);
        }

        let mut has_complete = false;
        let mut seen = std::collections::HashSet::new();

        for col in &self.columns {
            if !seen.insert(col.id.clone()) {
                return Err(BoardConfigError::DuplicateColumnId(col.id.clone()));
            }
            if col.counts_complete && col.is_exception {
                return Err(BoardConfigError::CompleteAndException(col.id.clone()));
            }
            if col.counts_complete {
                has_complete = true;
            }
        }

        if !has_complete {
            return Err(BoardConfigError::NoCompleteColumn);
        }

        Ok(())
    }

    pub fn column(&self, id: &ColumnId) -> Option<&ColumnDef> {
        self.columns.iter().find(|c| &c.id == id)
    }

    /// Unknown column ids are treated as incomplete, never as complete.
    pub fn counts_complete(&self, id: &ColumnId) -> bool {
        self.column(id)
            .is_some_and(|c| c.counts_complete && !c.is_exception)
    }

    pub fn is_exception(&self, id: &ColumnId) -> bool {
        self.column(id).is_some_and(|c| c.is_exception)
    }

    /// Tasks whose column_id matches no column in this board.
    pub fn orphans<'a>(&self, tasks: &'a [Task]) -> Vec<&'a Task> {
        tasks
            .iter()
            .filter(|t| self.column(&t.column_id).is_none())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_passes_validation() {
        let board = BoardConfig::factory(ProjectId::from_raw(1));
        assert!(board.validate().is_ok());
    }

    #[test]
    fn validate_rejects_empty_columns() {
        let board = BoardConfig {
            project_id: ProjectId::from_raw(1),
            columns: vec![],
        };
        assert!(matches!(
            board.validate(),
            Err(BoardConfigError::NoColumns)
        ));
    }

    #[test]
    fn validate_rejects_no_complete_column() {
        let board = BoardConfig {
            project_id: ProjectId::from_raw(1),
            columns: vec![ColumnDef {
                id: ColumnId::new("todo"),
                title: "Todo".into(),
                counts_complete: false,
                is_exception: false,
                accent: None,
                limit: None,
                order: 0,
            }],
        };
        assert!(matches!(
            board.validate(),
            Err(BoardConfigError::NoCompleteColumn)
        ));
    }

    #[test]
    fn validate_rejects_duplicate_ids() {
        let board = BoardConfig {
            project_id: ProjectId::from_raw(1),
            columns: vec![
                ColumnDef {
                    id: ColumnId::new("a"),
                    title: "A".into(),
                    counts_complete: true,
                    is_exception: false,
                    accent: None,
                    limit: None,
                    order: 0,
                },
                ColumnDef {
                    id: ColumnId::new("a"),
                    title: "A2".into(),
                    counts_complete: false,
                    is_exception: false,
                    accent: None,
                    limit: None,
                    order: 1,
                },
            ],
        };
        assert!(matches!(
            board.validate(),
            Err(BoardConfigError::DuplicateColumnId(_))
        ));
    }

    #[test]
    fn validate_rejects_complete_and_exception() {
        let board = BoardConfig {
            project_id: ProjectId::from_raw(1),
            columns: vec![ColumnDef {
                id: ColumnId::new("bad"),
                title: "Bad".into(),
                counts_complete: true,
                is_exception: true,
                accent: None,
                limit: None,
                order: 0,
            }],
        };
        assert!(matches!(
            board.validate(),
            Err(BoardConfigError::CompleteAndException(_))
        ));
    }
}
