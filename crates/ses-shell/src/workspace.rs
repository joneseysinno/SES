//! Workspace definitions and shell state.

use crate::ids::{ModuleId, WorkspaceId};
use crate::landmark::{LandmarkDef, LandmarkId};
use crate::page::{PageNode, PageTopBar};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Which pages the page-picker offers in this workspace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PageFilter {
    /// Seeded departmental workspace — picker shows only this module's pages.
    Department(ModuleId),
    /// User-created workspace — picker shows every page the user may access,
    /// grouped by department.
    UserDefined,
}

impl Default for PageFilter {
    fn default() -> Self {
        Self::UserDefined
    }
}

/// Context values every page in this workspace resolves against.
/// The Project department template binds `project_id` here; all its pages
/// then read the same project without each one asking separately.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceBinding {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub values: BTreeMap<String, String>,
}

impl WorkspaceBinding {
    pub const PROJECT_ID: &'static str = "project_id";
    pub const PROJECT_NUMBER: &'static str = "project_number";
    pub const PROJECT_NAME: &'static str = "project_name";

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.values.insert(key.into(), val.into());
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceDef {
    pub id: WorkspaceId,
    pub name: String,
    pub layout: PageNode,
    /// When set, the workspace area shows only this leaf (maximize).
    pub maximized: Option<crate::ids::LeafId>,
    /// Layout snapshot taken before maximize (for restore).
    #[serde(skip)]
    pub layout_before_maximize: Option<PageNode>,
    /// Optional sticky page top bar.
    #[serde(default)]
    pub top_bar: Option<PageTopBar>,
    /// Named landmarks pinned to the page scroll bar.
    #[serde(default)]
    pub landmarks: Vec<LandmarkDef>,
    /// Which pages the page-picker offers.
    #[serde(default)]
    pub page_filter: PageFilter,
    /// Context values pages resolve against.
    #[serde(default)]
    pub binding: WorkspaceBinding,
    /// True once the user has edited this workspace. Factory reseeding skips it.
    #[serde(default)]
    pub user_modified: bool,
    /// If this workspace was instantiated from a department template,
    /// which template it came from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_of: Option<ModuleId>,
    /// Stable factory seed key (e.g. `"project-mgmt/portfolio"`).
    /// User-created workspaces have `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_key: Option<String>,
    /// Deterministic factory tab order. Lower sorts first. Ties keep
    /// registration order. Overridden entirely once the user saves a
    /// startup profile with an explicit `workspace_order`.
    #[serde(default)]
    pub seed_order: i16,
}

impl WorkspaceDef {
    pub fn new(name: impl Into<String>, layout: PageNode) -> Self {
        Self {
            id: WorkspaceId::new(),
            name: name.into(),
            layout,
            maximized: None,
            layout_before_maximize: None,
            top_bar: None,
            landmarks: Vec::new(),
            page_filter: PageFilter::default(),
            binding: WorkspaceBinding::default(),
            user_modified: false,
            template_of: None,
            seed_key: None,
            seed_order: 0,
        }
    }

    pub fn with_top_bar(mut self, bar: PageTopBar) -> Self {
        self.top_bar = Some(bar);
        self
    }

    pub fn for_department(mut self, m: impl Into<ModuleId>) -> Self {
        self.page_filter = PageFilter::Department(m.into());
        self
    }

    pub fn bound_to(mut self, key: &str, val: impl Into<String>) -> Self {
        self.binding.set(key, val);
        self
    }

    pub fn with_seed_key(mut self, key: impl Into<String>) -> Self {
        self.seed_key = Some(key.into());
        self
    }

    pub fn with_seed_order(mut self, n: i16) -> Self {
        self.seed_order = n;
        self
    }

    /// Strip the department constraint — user takes ownership.
    pub fn into_custom(mut self) -> Self {
        self.page_filter = PageFilter::UserDefined;
        self.user_modified = true;
        self
    }

    pub fn add_landmark(&mut self, lm: LandmarkDef) {
        self.landmarks.push(lm);
    }

    pub fn remove_landmark(&mut self, id: LandmarkId) {
        self.landmarks.retain(|lm| lm.id != id);
    }

    pub fn display_layout(&self) -> &PageNode {
        if let Some(id) = self.maximized {
            if let Some(leaf) = self.layout.find_leaf(id) {
                // Caller should use maximized_as_node for rendering; this returns full tree.
                let _ = leaf;
            }
        }
        &self.layout
    }
}

const MAX_PENDING_TOP_BAR_ACTIONS: usize = 64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellState {
    pub workspaces: Vec<WorkspaceDef>,
    pub active_workspace: WorkspaceId,
    pub status_message: String,
    /// Queued page top-bar action ids keyed by workspace.
    /// Department pages drain only their own workspace's actions.
    #[serde(default, skip)]
    pub pending_top_bar_actions: Vec<(WorkspaceId, String)>,
}

impl ShellState {
    pub fn push_top_bar_action(&mut self, ws: WorkspaceId, action_id: impl Into<String>) {
        self.pending_top_bar_actions.push((ws, action_id.into()));
        while self.pending_top_bar_actions.len() > MAX_PENDING_TOP_BAR_ACTIONS {
            self.pending_top_bar_actions.remove(0);
        }
    }

    /// Drain only the actions queued for `ws`. Other workspaces' actions
    /// survive — a page that is not mounted must not lose its mail.
    pub fn take_top_bar_actions_for(&mut self, ws: WorkspaceId) -> Vec<String> {
        let (mine, theirs): (Vec<_>, Vec<_>) =
            std::mem::take(&mut self.pending_top_bar_actions)
                .into_iter()
                .partition(|(id, _)| *id == ws);
        self.pending_top_bar_actions = theirs;
        mine.into_iter().map(|(_, a)| a).collect()
    }

    /// A workspace instantiated from `module` whose binding `key` equals `value`.
    pub fn find_bound(&self, module: &ModuleId, key: &str, value: &str) -> Option<WorkspaceId> {
        self.workspaces
            .iter()
            .find(|w| w.template_of.as_ref() == Some(module) && w.binding.get(key) == Some(value))
            .map(|w| w.id)
    }

    pub fn active(&self) -> Option<&WorkspaceDef> {
        self.workspaces
            .iter()
            .find(|w| w.id == self.active_workspace)
    }

    pub fn active_mut(&mut self) -> Option<&mut WorkspaceDef> {
        let id = self.active_workspace;
        self.workspaces.iter_mut().find(|w| w.id == id)
    }

    pub fn set_active(&mut self, id: WorkspaceId) -> bool {
        if self.workspaces.iter().any(|w| w.id == id) {
            self.active_workspace = id;
            true
        } else {
            false
        }
    }

    pub fn add_workspace(&mut self, workspace: WorkspaceDef) {
        let id = workspace.id;
        self.workspaces.push(workspace);
        self.active_workspace = id;
    }

    pub fn remove_workspace(&mut self, id: WorkspaceId) -> bool {
        if self.workspaces.len() <= 1 {
            return false;
        }
        let Some(idx) = self.workspaces.iter().position(|w| w.id == id) else {
            return false;
        };
        self.workspaces.remove(idx);
        if self.active_workspace == id {
            let new_idx = idx.min(self.workspaces.len() - 1);
            self.active_workspace = self.workspaces[new_idx].id;
        }
        true
    }

    pub fn rename_workspace(&mut self, id: WorkspaceId, name: impl Into<String>) -> bool {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return false;
        }
        let Some(ws) = self.workspaces.iter_mut().find(|w| w.id == id) else {
            return false;
        };
        ws.name = trimmed.to_string();
        true
    }

    pub fn duplicate_workspace(&mut self, id: WorkspaceId) -> bool {
        let Some(current) = self.workspaces.iter().find(|w| w.id == id).cloned() else {
            return false;
        };
        let mut dup = WorkspaceDef::new(
            format!("{} Copy", current.name),
            current.layout.clone(),
        );
        dup.maximized = None;
        dup.layout_before_maximize = None;
        dup.top_bar = current.top_bar.clone();
        dup.landmarks = current.landmarks.clone();
        dup.page_filter = current.page_filter.clone();
        dup.binding = current.binding.clone();
        dup.user_modified = true;
        dup.template_of = current.template_of.clone();
        dup.seed_key = None;
        dup.seed_order = current.seed_order;
        self.add_workspace(dup);
        true
    }

    pub fn duplicate_active(&mut self) {
        let id = self.active_workspace;
        let _ = self.duplicate_workspace(id);
    }

    /// Move the workspace at `from` to index `to` (clamped). Active id is unchanged.
    pub fn reorder_workspace(&mut self, from: usize, to: usize) -> bool {
        let len = self.workspaces.len();
        if from >= len || len <= 1 {
            return false;
        }
        let to = to.min(len - 1);
        if from == to {
            return true;
        }
        let item = self.workspaces.remove(from);
        self.workspaces.insert(to, item);
        true
    }

    pub fn workspace_index(&self, id: WorkspaceId) -> Option<usize> {
        self.workspaces.iter().position(|w| w.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::{PageDescriptor, PageNode};

    fn blank(name: &str) -> WorkspaceDef {
        WorkspaceDef::new(
            name,
            PageNode::leaf(PageDescriptor::new("core-ui", "view")),
        )
    }

    #[test]
    fn rename_trims_and_rejects_empty() {
        let mut s = ShellState {
            workspaces: vec![blank("A")],
            active_workspace: WorkspaceId::new(),
            status_message: String::new(),
            pending_top_bar_actions: Vec::new(),
        };
        let id = s.workspaces[0].id;
        s.active_workspace = id;
        assert!(s.rename_workspace(id, "  Layout  "));
        assert_eq!(s.workspaces[0].name, "Layout");
        assert!(!s.rename_workspace(id, "   "));
        assert_eq!(s.workspaces[0].name, "Layout");
    }

    #[test]
    fn duplicate_by_id() {
        let a = blank("Layout");
        let id = a.id;
        let mut s = ShellState {
            workspaces: vec![a],
            active_workspace: id,
            status_message: String::new(),
            pending_top_bar_actions: Vec::new(),
        };
        assert!(s.duplicate_workspace(id));
        assert_eq!(s.workspaces.len(), 2);
        assert_eq!(s.workspaces[1].name, "Layout Copy");
        assert_eq!(s.active_workspace, s.workspaces[1].id);
    }

    #[test]
    fn reorder_keeps_active_id() {
        let a = blank("A");
        let b = blank("B");
        let c = blank("C");
        let active = b.id;
        let mut s = ShellState {
            workspaces: vec![a, b, c],
            active_workspace: active,
            status_message: String::new(),
            pending_top_bar_actions: Vec::new(),
        };
        assert!(s.reorder_workspace(0, 2));
        assert_eq!(
            s.workspaces
                .iter()
                .map(|w| w.name.as_str())
                .collect::<Vec<_>>(),
            vec!["B", "C", "A"]
        );
        assert_eq!(s.active_workspace, active);
    }

    #[test]
    fn serde_defaults_for_new_fields() {
        let json = r#"{
            "id": 1,
            "name": "Legacy",
            "layout": {
                "Leaf": {
                    "id": 2,
                    "page": { "module_id": "core-ui", "page_id": "view" },
                    "io": {
                        "show_input": false,
                        "show_output": false,
                        "placement": "Below",
                        "channel": null
                    }
                }
            },
            "maximized": null
        }"#;
        let ws: WorkspaceDef = serde_json::from_str(json).expect("decode");
        assert_eq!(ws.page_filter, PageFilter::UserDefined);
        assert!(ws.binding.values.is_empty());
        assert!(!ws.user_modified);
        assert!(ws.seed_key.is_none());
        assert_eq!(ws.seed_order, 0);
    }

    #[test]
    fn action_queue_is_per_workspace() {
        let a = blank("A");
        let b = blank("B");
        let id_a = a.id;
        let id_b = b.id;
        let mut s = ShellState {
            workspaces: vec![a, b],
            active_workspace: id_a,
            status_message: String::new(),
            pending_top_bar_actions: Vec::new(),
        };
        s.push_top_bar_action(id_a, "new-project");
        s.push_top_bar_action(id_b, "new-proposal");
        assert_eq!(s.take_top_bar_actions_for(id_a), vec!["new-project".to_string()]);
        assert_eq!(s.take_top_bar_actions_for(id_a), Vec::<String>::new());
        assert_eq!(s.take_top_bar_actions_for(id_b), vec!["new-proposal".to_string()]);
    }

    #[test]
    fn action_queue_caps_at_64_dropping_oldest() {
        let a = blank("A");
        let id = a.id;
        let mut s = ShellState {
            workspaces: vec![a],
            active_workspace: id,
            status_message: String::new(),
            pending_top_bar_actions: Vec::new(),
        };
        for i in 0..70 {
            s.push_top_bar_action(id, format!("a{i}"));
        }
        assert_eq!(s.pending_top_bar_actions.len(), 64);
        assert_eq!(s.pending_top_bar_actions[0].1, "a6");
        assert_eq!(s.pending_top_bar_actions[63].1, "a69");
    }

    #[test]
    fn find_bound_requires_template_of() {
        let module = ModuleId::new("project");
        let mut bound = blank("Bound");
        bound.template_of = Some(module.clone());
        bound.binding.set(WorkspaceBinding::PROJECT_ID, "7");
        let mut unbound = blank("Unbound");
        unbound.binding.set(WorkspaceBinding::PROJECT_ID, "7");
        let s = ShellState {
            workspaces: vec![bound.clone(), unbound],
            active_workspace: bound.id,
            status_message: String::new(),
            pending_top_bar_actions: Vec::new(),
        };
        assert_eq!(
            s.find_bound(&module, WorkspaceBinding::PROJECT_ID, "7"),
            Some(bound.id)
        );
        assert!(s
            .find_bound(&ModuleId::new("other"), WorkspaceBinding::PROJECT_ID, "7")
            .is_none());
    }
}
