//! In-memory department store implementing bridge command/query contracts.
//!
//! Persistence is layered on by ses-app (infinite-db). Progress is always
//! recomputed — never stored.

use crate::project::bridge::{
    reassign_cards, NewBoardCardParams, NewDocRefParams, NewMilestoneParams, NewTaskParams,
    NewTimeEntryParams, ProjectCommand, ProjectQuery,
};
use crate::project::ui::workspace::for_project;
use crate::project::payloads::{
    BoardCard, BoardConfig, BoardConfigError, ChecklistItem, ColumnId, DocCategory, DocRef,
    Milestone, Priority, Task, TimeEntry,
};
use crate::project::progress::{compute, ProjectProgress};
use crate::project_management::bridge::{NewProjectParams, ProjectMgmtCommand, ProjectMgmtQuery};
use crate::project_management::payloads::{
    ProjectPhase, ProjectRecord, Proposal, ProposalStatus,
};
use crate::shared::{
    reset_id_counter, Address, BoardCardId, Client, DocRefId, MilestoneId, Minutes, Money,
    ProjectId, ProposalId, TaskId, TimeEntryId,
};
use dioxus::prelude::*;
use ses_adapter::payload::{DesignBasis, ProjectStatus, RiskCategory, UnitSystemPref};
use ses_shell::WorkspaceDef;
use std::collections::BTreeMap;
use thiserror::Error;

/// Dioxus context for the shared department store.
pub type DeptStoreCtx = Signal<DeptStore>;

pub fn use_dept_store() -> DeptStoreCtx {
    use_context::<DeptStoreCtx>()
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum StoreError {
    #[error("{0}")]
    Message(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    Board(#[from] BoardConfigError),
}

impl StoreError {
    fn msg(s: impl Into<String>) -> Self {
        Self::Message(s.into())
    }
}

/// Side effects that the UI layer must apply (shell, navigation).
#[derive(Debug, Clone)]
pub enum StoreEffect {
    None,
    /// Open a bound Project department workspace.
    OpenWorkspace(WorkspaceDef),
}

/// Query results for project-management.
#[derive(Debug, Clone)]
pub enum MgmtQueryResult {
    Projects(Vec<ProjectRecord>),
    Project(ProjectRecord),
    Proposals(Vec<Proposal>),
    PortfolioMetrics {
        active_count: u32,
        contract_cents: i64,
    },
    PortfolioProgress(BTreeMap<ProjectId, ProjectProgress>),
}

/// Query results for the Project department.
#[derive(Debug, Clone)]
pub enum ProjectQueryResult {
    BoardCards(Vec<BoardCard>),
    Tasks(Vec<Task>),
    Task(Task),
    Board(BoardConfig),
    TimeEntries(Vec<TimeEntry>),
    Docs(Vec<DocRef>),
    Milestones(Vec<Milestone>),
    Progress(ProjectProgress),
}

/// Durable department working set (everything except Ephemeral progress).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeptStore {
    pub projects: BTreeMap<ProjectId, ProjectRecord>,
    pub boards: BTreeMap<ProjectId, BoardConfig>,
    pub board_cards: BTreeMap<BoardCardId, BoardCard>,
    pub tasks: BTreeMap<TaskId, Task>,
    pub time_entries: BTreeMap<TimeEntryId, TimeEntry>,
    pub docs: BTreeMap<DocRefId, DocRef>,
    pub milestones: BTreeMap<MilestoneId, Milestone>,
    pub proposals: BTreeMap<ProposalId, Proposal>,
}

impl DeptStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Empty store for production; callers may seed after load.
    pub fn empty() -> Self {
        Self::new()
    }

    /// Seed one demo project + factory board when the DB is empty (dev UX).
    pub fn with_demo_seed() -> Self {
        let mut store = Self::new();
        store
            .execute_mgmt(ProjectMgmtCommand::CreateProject(NewProjectParams {
                name: "Clinic Addition".into(),
                number: "2026-001".into(),
            }))
            .expect("demo seed");
        let pid = *store.projects.keys().next().expect("seeded project");
        if let Some(p) = store.projects.get_mut(&pid) {
            p.client = Client::from_name("Example Health");
            p.address = Address::from_freeform("Salt Lake City, UT");
            p.status = ProjectStatus::Active;
            p.phase = ProjectPhase::InProgress;
            p.manager = "Alex PM".into();
            p.contract_value = Some(Money::usd(250_000_00));
            p.engineer_of_record = "Dana, PE".into();
        }
        let in_design = ColumnId::new("in-design");
        let card_id = BoardCardId::new();
        store.board_cards.insert(
            card_id,
            BoardCard {
                id: card_id,
                project_id: pid,
                column_id: in_design,
                title: "Design Criteria".into(),
                order: 0,
            },
        );
        for (i, title) in ["Snow", "Rain", "Seismic", "Wind"].iter().enumerate() {
            let id = TaskId::new();
            store.tasks.insert(
                id,
                Task {
                    id,
                    project_id: pid,
                    card_id,
                    title: (*title).into(),
                    description: String::new(),
                    estimate: Minutes::from_hours((i as u32 + 1) * 2),
                    assignee: Some("Demo User".into()),
                    due_utc: None,
                    priority: Priority::Normal,
                    checklist: vec![ChecklistItem {
                        label: format!("{title} subtask"),
                        done: false,
                    }],
                    order: i as u32,
                    created_utc: now_utc(),
                    completed_utc: None,
                },
            );
        }
        store
    }

    /// Bump the shared id counter past any loaded ids.
    pub fn bump_ids(&self) {
        let mut max = 1u64;
        for id in self.projects.keys() {
            max = max.max(id.0);
        }
        for id in self.board_cards.keys() {
            max = max.max(id.0);
        }
        for id in self.tasks.keys() {
            max = max.max(id.0);
        }
        for id in self.time_entries.keys() {
            max = max.max(id.0);
        }
        for id in self.docs.keys() {
            max = max.max(id.0);
        }
        for id in self.milestones.keys() {
            max = max.max(id.0);
        }
        for id in self.proposals.keys() {
            max = max.max(id.0);
        }
        reset_id_counter(max.saturating_add(1));
    }

    pub fn execute_mgmt(
        &mut self,
        cmd: ProjectMgmtCommand,
    ) -> Result<StoreEffect, StoreError> {
        match cmd {
            ProjectMgmtCommand::CreateProject(params) => {
                let id = ProjectId::new();
                let now = now_utc();
                let record = ProjectRecord {
                    id,
                    name: params.name,
                    number: params.number,
                    client: Client::from_name(""),
                    address: Address::default(),
                    status: ProjectStatus::Draft,
                    phase: ProjectPhase::Prospect,
                    manager: String::new(),
                    start_utc: now,
                    target_finish_utc: None,
                    contract_value: None,
                    design_basis: empty_design_basis(),
                    engineer_of_record: String::new(),
                    created_utc: now,
                };
                let board = BoardConfig::factory(id);
                board.validate()?;
                self.projects.insert(id, record);
                self.boards.insert(id, board);
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::UpdateProject { id, patch } => {
                let p = self
                    .projects
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("project {id}")))?;
                if let Some(name) = patch.name {
                    p.name = name;
                }
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::SetPhase { id, phase } => {
                let p = self
                    .projects
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("project {id}")))?;
                p.phase = phase;
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::SetStatus { id, status } => {
                let p = self
                    .projects
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("project {id}")))?;
                p.status = status;
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::ArchiveProject { id } => {
                let p = self
                    .projects
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("project {id}")))?;
                p.status = ProjectStatus::Archived;
                p.phase = ProjectPhase::Closed;
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::CreateProposal(params) => {
                if !self.projects.contains_key(&params.project_id) {
                    return Err(StoreError::NotFound(format!(
                        "project {}",
                        params.project_id
                    )));
                }
                let id = ProposalId::new();
                self.proposals.insert(
                    id,
                    Proposal {
                        id,
                        project_id: params.project_id,
                        revision: 1,
                        scope: params.scope,
                        fee: Money::usd(0),
                        schedule_weeks: 0,
                        status: ProposalStatus::Draft,
                        sent_utc: None,
                    },
                );
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::ReviseProposal { id, patch } => {
                let prop = self
                    .proposals
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("proposal {id}")))?;
                if let Some(scope) = patch.scope {
                    prop.scope = scope;
                }
                prop.revision = prop.revision.saturating_add(1);
                prop.status = ProposalStatus::Draft;
                prop.sent_utc = None;
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::SendProposal { id } => {
                let prop = self
                    .proposals
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("proposal {id}")))?;
                prop.status = ProposalStatus::Sent;
                prop.sent_utc = Some(now_utc());
                Ok(StoreEffect::None)
            }
            ProjectMgmtCommand::OpenProjectWorkspace { project_id } => {
                let p = self
                    .projects
                    .get(&project_id)
                    .ok_or_else(|| StoreError::NotFound(format!("project {project_id}")))?;
                let ws = for_project(project_id, &p.name);
                Ok(StoreEffect::OpenWorkspace(ws))
            }
        }
    }

    pub fn query_mgmt(&self, q: ProjectMgmtQuery) -> Result<MgmtQueryResult, StoreError> {
        match q {
            ProjectMgmtQuery::ListAll { filter: _ } => Ok(MgmtQueryResult::Projects(
                self.projects.values().cloned().collect(),
            )),
            ProjectMgmtQuery::GetProject(id) => self
                .projects
                .get(&id)
                .cloned()
                .map(MgmtQueryResult::Project)
                .ok_or_else(|| StoreError::NotFound(format!("project {id}"))),
            ProjectMgmtQuery::ListProposals(pid) => Ok(MgmtQueryResult::Proposals(
                self.proposals
                    .values()
                    .filter(|p| p.project_id == pid)
                    .cloned()
                    .collect(),
            )),
            ProjectMgmtQuery::PortfolioMetrics => {
                let active_count = self
                    .projects
                    .values()
                    .filter(|p| p.status == ProjectStatus::Active)
                    .count() as u32;
                let contract_cents = self
                    .projects
                    .values()
                    .filter_map(|p| p.contract_value.as_ref())
                    .map(|m| m.cents)
                    .fold(0i64, i64::saturating_add);
                Ok(MgmtQueryResult::PortfolioMetrics {
                    active_count,
                    contract_cents,
                })
            }
            ProjectMgmtQuery::PortfolioProgress => {
                let mut map = BTreeMap::new();
                for pid in self.projects.keys() {
                    map.insert(*pid, self.progress_for(*pid));
                }
                Ok(MgmtQueryResult::PortfolioProgress(map))
            }
        }
    }

    pub fn execute_project(&mut self, cmd: ProjectCommand) -> Result<StoreEffect, StoreError> {
        match cmd {
            ProjectCommand::CreateBoardCard(NewBoardCardParams {
                project_id,
                title,
                column_id,
            }) => {
                self.require_project(project_id)?;
                let board = self.board(project_id)?;
                if board.column(&column_id).is_none() {
                    return Err(StoreError::msg(format!("unknown column {}", column_id.0)));
                }
                let id = BoardCardId::new();
                let order = self
                    .board_cards
                    .values()
                    .filter(|c| c.project_id == project_id)
                    .map(|c| c.order)
                    .max()
                    .map(|o| o.saturating_add(1))
                    .unwrap_or(0);
                self.board_cards.insert(
                    id,
                    BoardCard {
                        id,
                        project_id,
                        column_id,
                        title,
                        order,
                    },
                );
                Ok(StoreEffect::None)
            }
            ProjectCommand::MoveBoardCard {
                id,
                column_id,
                order,
            } => {
                let card = self
                    .board_cards
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("card {id}")))?;
                let board = self
                    .boards
                    .get(&card.project_id)
                    .ok_or_else(|| StoreError::NotFound("board".into()))?;
                if board.column(&column_id).is_none() {
                    return Err(StoreError::msg(format!(
                        "unknown column {}",
                        column_id.0
                    )));
                }
                card.column_id = column_id;
                card.order = order;
                Ok(StoreEffect::None)
            }
            ProjectCommand::CreateTask(NewTaskParams {
                project_id,
                card_id,
                title,
            }) => {
                self.require_project(project_id)?;
                let card = self
                    .board_cards
                    .get(&card_id)
                    .ok_or_else(|| StoreError::NotFound(format!("card {card_id}")))?;
                if card.project_id != project_id {
                    return Err(StoreError::msg("card belongs to another project"));
                }
                let id = TaskId::new();
                let order = self
                    .tasks
                    .values()
                    .filter(|t| t.card_id == card_id)
                    .map(|t| t.order)
                    .max()
                    .map(|o| o.saturating_add(1))
                    .unwrap_or(0);
                self.tasks.insert(
                    id,
                    Task {
                        id,
                        project_id,
                        card_id,
                        title,
                        description: String::new(),
                        estimate: Minutes(0),
                        assignee: None,
                        due_utc: None,
                        priority: Priority::Normal,
                        checklist: vec![],
                        order,
                        created_utc: now_utc(),
                        completed_utc: None,
                    },
                );
                Ok(StoreEffect::None)
            }
            ProjectCommand::UpdateTask { id, patch } => {
                let t = self
                    .tasks
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("task {id}")))?;
                if let Some(title) = patch.title {
                    t.title = title;
                }
                Ok(StoreEffect::None)
            }
            ProjectCommand::ToggleTaskComplete { task_id } => {
                let t = self
                    .tasks
                    .get_mut(&task_id)
                    .ok_or_else(|| StoreError::NotFound(format!("task {task_id}")))?;
                if t.completed_utc.is_some() {
                    t.completed_utc = None;
                } else {
                    t.completed_utc = Some(now_utc());
                }
                Ok(StoreEffect::None)
            }
            ProjectCommand::ToggleChecklistItem { task_id, index } => {
                let t = self
                    .tasks
                    .get_mut(&task_id)
                    .ok_or_else(|| StoreError::NotFound(format!("task {task_id}")))?;
                let item = t
                    .checklist
                    .get_mut(index)
                    .ok_or_else(|| StoreError::msg("checklist index out of range"))?;
                item.done = !item.done;
                Ok(StoreEffect::None)
            }
            ProjectCommand::AddChecklistItem { task_id, label } => {
                let t = self
                    .tasks
                    .get_mut(&task_id)
                    .ok_or_else(|| StoreError::NotFound(format!("task {task_id}")))?;
                t.checklist.push(ChecklistItem {
                    label,
                    done: false,
                });
                Ok(StoreEffect::None)
            }
            ProjectCommand::DeleteTask(id) => {
                self.tasks
                    .remove(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("task {id}")))?;
                Ok(StoreEffect::None)
            }
            ProjectCommand::LogTime(NewTimeEntryParams {
                project_id,
                task_id,
                minutes,
                billable,
            }) => {
                self.require_project(project_id)?;
                let id = TimeEntryId::new();
                self.time_entries.insert(
                    id,
                    TimeEntry {
                        id,
                        project_id,
                        task_id,
                        who: String::new(),
                        minutes,
                        date_utc: now_utc(),
                        note: String::new(),
                        billable,
                    },
                );
                Ok(StoreEffect::None)
            }
            ProjectCommand::AddDocRef(NewDocRefParams { project_id, title }) => {
                self.require_project(project_id)?;
                let id = DocRefId::new();
                self.docs.insert(
                    id,
                    DocRef {
                        id,
                        project_id,
                        title,
                        category: DocCategory::Other,
                        uri: String::new(),
                        revision: "A".into(),
                        added_utc: now_utc(),
                        added_by: String::new(),
                    },
                );
                Ok(StoreEffect::None)
            }
            ProjectCommand::CreateMilestone(NewMilestoneParams {
                project_id,
                title,
                target_utc,
            }) => {
                self.require_project(project_id)?;
                let id = MilestoneId::new();
                self.milestones.insert(
                    id,
                    Milestone {
                        id,
                        project_id,
                        title,
                        target_utc,
                        actual_utc: None,
                        gating_tasks: vec![],
                    },
                );
                Ok(StoreEffect::None)
            }
            ProjectCommand::SetMilestoneActual { id, actual_utc } => {
                let m = self
                    .milestones
                    .get_mut(&id)
                    .ok_or_else(|| StoreError::NotFound(format!("milestone {id}")))?;
                m.actual_utc = Some(actual_utc);
                Ok(StoreEffect::None)
            }
            ProjectCommand::AddColumn {
                project_id,
                column,
            } => {
                let board = self
                    .boards
                    .get_mut(&project_id)
                    .ok_or_else(|| StoreError::NotFound(format!("board {project_id}")))?;
                board.columns.push(column);
                board.validate()?;
                Ok(StoreEffect::None)
            }
            ProjectCommand::UpdateColumn {
                project_id,
                column,
            } => {
                let board = self
                    .boards
                    .get_mut(&project_id)
                    .ok_or_else(|| StoreError::NotFound(format!("board {project_id}")))?;
                if let Some(slot) = board.columns.iter_mut().find(|c| c.id == column.id) {
                    *slot = column;
                } else {
                    return Err(StoreError::NotFound(format!("column {}", column.id.0)));
                }
                board.validate()?;
                Ok(StoreEffect::None)
            }
            ProjectCommand::ReorderColumns { project_id, order } => {
                let board = self
                    .boards
                    .get_mut(&project_id)
                    .ok_or_else(|| StoreError::NotFound(format!("board {project_id}")))?;
                let mut next = Vec::with_capacity(order.len());
                for id in &order {
                    let col = board
                        .column(id)
                        .cloned()
                        .ok_or_else(|| StoreError::NotFound(format!("column {}", id.0)))?;
                    next.push(col);
                }
                if next.len() != board.columns.len() {
                    return Err(StoreError::msg("reorder must include every column"));
                }
                for (i, c) in next.iter_mut().enumerate() {
                    c.order = i as u16;
                }
                board.columns = next;
                board.validate()?;
                Ok(StoreEffect::None)
            }
            ProjectCommand::RemoveColumn {
                project_id,
                id,
                reassign_to,
            } => {
                if id == reassign_to {
                    return Err(StoreError::msg("reassign_to must differ from removed column"));
                }
                let board = self
                    .boards
                    .get(&project_id)
                    .ok_or_else(|| StoreError::NotFound(format!("board {project_id}")))?
                    .clone();
                if board.column(&reassign_to).is_none() {
                    return Err(StoreError::NotFound(format!("column {}", reassign_to.0)));
                }
                let mut next = board.clone();
                next.columns.retain(|c| c.id != id);
                next.validate()?;
                let project_cards: Vec<BoardCard> = self
                    .board_cards
                    .values()
                    .filter(|c| c.project_id == project_id)
                    .cloned()
                    .collect();
                let updated = reassign_cards(&project_cards, &id, &reassign_to);
                for c in updated {
                    self.board_cards.insert(c.id, c);
                }
                self.boards.insert(project_id, next);
                Ok(StoreEffect::None)
            }
            ProjectCommand::ResetBoardToFactory { project_id } => {
                self.require_project(project_id)?;
                let board = BoardConfig::factory(project_id);
                board.validate()?;
                let first = board.columns[0].id.clone();
                for c in self
                    .board_cards
                    .values_mut()
                    .filter(|c| c.project_id == project_id)
                {
                    if board.column(&c.column_id).is_none() {
                        c.column_id = first.clone();
                    }
                }
                self.boards.insert(project_id, board);
                Ok(StoreEffect::None)
            }
        }
    }

    pub fn query_project(&self, q: ProjectQuery) -> Result<ProjectQueryResult, StoreError> {
        match q {
            ProjectQuery::ListBoardCards { project_id } => Ok(ProjectQueryResult::BoardCards(
                self.board_cards
                    .values()
                    .filter(|c| c.project_id == project_id)
                    .cloned()
                    .collect(),
            )),
            ProjectQuery::ListTasks {
                project_id,
                filter: _,
            } => Ok(ProjectQueryResult::Tasks(
                self.tasks
                    .values()
                    .filter(|t| t.project_id == project_id)
                    .cloned()
                    .collect(),
            )),
            ProjectQuery::GetTask(id) => self
                .tasks
                .get(&id)
                .cloned()
                .map(ProjectQueryResult::Task)
                .ok_or_else(|| StoreError::NotFound(format!("task {id}"))),
            ProjectQuery::GetBoardConfig(pid) => self
                .boards
                .get(&pid)
                .cloned()
                .map(ProjectQueryResult::Board)
                .ok_or_else(|| StoreError::NotFound(format!("board {pid}"))),
            ProjectQuery::ListTimeEntries { project_id, range } => {
                Ok(ProjectQueryResult::TimeEntries(
                    self.time_entries
                        .values()
                        .filter(|e| {
                            e.project_id == project_id
                                && e.date_utc >= range.start_utc
                                && e.date_utc <= range.end_utc
                        })
                        .cloned()
                        .collect(),
                ))
            }
            ProjectQuery::ListDocs(pid) => Ok(ProjectQueryResult::Docs(
                self.docs
                    .values()
                    .filter(|d| d.project_id == pid)
                    .cloned()
                    .collect(),
            )),
            ProjectQuery::ListMilestones(pid) => Ok(ProjectQueryResult::Milestones(
                self.milestones
                    .values()
                    .filter(|m| m.project_id == pid)
                    .cloned()
                    .collect(),
            )),
            ProjectQuery::Progress(pid) => Ok(ProjectQueryResult::Progress(self.progress_for(pid))),
        }
    }

    fn progress_for(&self, project_id: ProjectId) -> ProjectProgress {
        let board = self
            .boards
            .get(&project_id)
            .cloned()
            .unwrap_or_else(|| BoardConfig::factory(project_id));
        let cards: Vec<_> = self
            .board_cards
            .values()
            .filter(|c| c.project_id == project_id)
            .cloned()
            .collect();
        let tasks: Vec<_> = self
            .tasks
            .values()
            .filter(|t| t.project_id == project_id)
            .cloned()
            .collect();
        let entries: Vec<_> = self
            .time_entries
            .values()
            .filter(|e| e.project_id == project_id)
            .cloned()
            .collect();
        compute(&board, &cards, &tasks, &entries)
    }

    fn require_project(&self, id: ProjectId) -> Result<(), StoreError> {
        if self.projects.contains_key(&id) {
            Ok(())
        } else {
            Err(StoreError::NotFound(format!("project {id}")))
        }
    }

    fn board(&self, project_id: ProjectId) -> Result<&BoardConfig, StoreError> {
        self.boards
            .get(&project_id)
            .ok_or_else(|| StoreError::NotFound(format!("board {project_id}")))
    }
}

fn empty_design_basis() -> DesignBasis {
    DesignBasis {
        code_stack: vec![],
        amendment_branch: None,
        display_units: UnitSystemPref::Imperial,
        sds_milli: 0,
        sd1_milli: 0,
        seismic_design_category: String::new(),
        risk_category: RiskCategory::Ii,
    }
}

fn now_utc() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::payloads::{ColumnDef, ColumnId};

    #[test]
    fn create_project_seeds_board() {
        let mut store = DeptStore::new();
        store
            .execute_mgmt(ProjectMgmtCommand::CreateProject(NewProjectParams {
                name: "A".into(),
                number: "1".into(),
            }))
            .unwrap();
        assert_eq!(store.projects.len(), 1);
        let pid = *store.projects.keys().next().unwrap();
        assert!(store.boards.contains_key(&pid));
        store.boards[&pid].validate().unwrap();
    }

    #[test]
    fn remove_column_reassigns_and_validates() {
        let mut store = DeptStore::with_demo_seed();
        let pid = *store.projects.keys().next().unwrap();
        store
            .execute_project(ProjectCommand::AddColumn {
                project_id: pid,
                column: ColumnDef {
                    id: ColumnId::new("blocked"),
                    title: "Blocked".into(),
                    counts_complete: false,
                    is_exception: true,
                    accent: Some("warn".into()),
                    limit: None,
                    order: 10,
                },
            })
            .unwrap();
        let card_id = *store
            .board_cards
            .values()
            .find(|c| c.project_id == pid)
            .map(|c| &c.id)
            .unwrap();
        store
            .execute_project(ProjectCommand::MoveBoardCard {
                id: card_id,
                column_id: ColumnId::new("blocked"),
                order: 0,
            })
            .unwrap();
        store
            .execute_project(ProjectCommand::RemoveColumn {
                project_id: pid,
                id: ColumnId::new("blocked"),
                reassign_to: ColumnId::new("proposals"),
            })
            .unwrap();
        let board = &store.boards[&pid];
        assert!(board.column(&ColumnId::new("blocked")).is_none());
        for c in store.board_cards.values().filter(|c| c.project_id == pid) {
            assert_ne!(c.column_id.0, "blocked");
        }
    }

    #[test]
    fn portfolio_progress_is_ephemeral_compute() {
        let store = DeptStore::with_demo_seed();
        let MgmtQueryResult::PortfolioProgress(map) = store
            .query_mgmt(ProjectMgmtQuery::PortfolioProgress)
            .unwrap()
        else {
            panic!();
        };
        assert!(!map.is_empty());
        let p = map.values().next().unwrap();
        assert!(p.total_task_count > 0);
    }

    #[test]
    fn open_workspace_effect() {
        let mut store = DeptStore::with_demo_seed();
        let pid = *store.projects.keys().next().unwrap();
        let StoreEffect::OpenWorkspace(ws) = store
            .execute_mgmt(ProjectMgmtCommand::OpenProjectWorkspace { project_id: pid })
            .unwrap()
        else {
            panic!("expected OpenWorkspace");
        };
        assert_eq!(ws.binding.get("project_id"), Some(pid.0.to_string().as_str()));
    }
}
