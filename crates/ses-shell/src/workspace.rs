//! Workspace definitions and shell state.

use crate::ids::WorkspaceId;
use crate::landmark::{LandmarkDef, LandmarkId};
use crate::page::{PageNode, PageTopBar};
use serde::{Deserialize, Serialize};

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
        }
    }

    pub fn with_top_bar(mut self, bar: PageTopBar) -> Self {
        self.top_bar = Some(bar);
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellState {
    pub workspaces: Vec<WorkspaceDef>,
    pub active_workspace: WorkspaceId,
    pub status_message: String,
}

impl ShellState {
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
    use crate::page::PageNode;
    use crate::pod::{PodDescriptor, PodKind};

    fn blank(name: &str) -> WorkspaceDef {
        WorkspaceDef::new(
            name,
            PageNode::leaf(PodDescriptor::new(PodKind::View, "core-ui")),
        )
    }

    #[test]
    fn rename_trims_and_rejects_empty() {
        let mut s = ShellState {
            workspaces: vec![blank("A")],
            active_workspace: WorkspaceId::new(),
            status_message: String::new(),
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
}
