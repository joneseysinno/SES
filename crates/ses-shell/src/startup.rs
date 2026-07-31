//! What the app opens with. Persisted separately from workspace layouts
//! so resetting one does not destroy the other.

use crate::ids::{ModuleId, WorkspaceId};
use crate::workspace::{ShellState, WorkspaceDef};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Migration marker. Bump when a release must rewrite persisted state.
pub const CURRENT_SCHEMA_VERSION: u16 = 1;

/// Seed keys from the scaffolding era. Dropped once, at v0 → v1.
const RETIRED_SEED_KEYS: &[&str] = &[
    "core/layout",
    "core/analysis",
    "core/documentation",
    "project/template", // was seeded as a tab; is a template
];

/// User-owned startup defaults. Factory seed is overlayed underneath.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartupProfile {
    /// Workspaces the user wants at launch, in tab order.
    /// Empty ⇒ fall back to factory seed order.
    pub workspace_order: Vec<WorkspaceId>,
    /// Which workspace is active on launch.
    pub active_workspace: Option<WorkspaceId>,
    /// Department modules enabled at launch.
    pub enabled_modules: Vec<ModuleId>,
    /// Factory workspaces the user explicitly deleted. Never reseeded.
    pub suppressed_factory: Vec<String>,
    /// When true, factory seeding is skipped entirely on launch.
    pub factory_seed_disabled: bool,
    /// Migration marker. Bump when a release must rewrite persisted state.
    #[serde(default)]
    pub schema_version: u16,
}

impl Default for StartupProfile {
    fn default() -> Self {
        Self {
            workspace_order: Vec::new(),
            active_workspace: None,
            enabled_modules: Vec::new(),
            suppressed_factory: Vec::new(),
            factory_seed_disabled: false,
            schema_version: CURRENT_SCHEMA_VERSION,
        }
    }
}

/// One-time rewrite of persisted state. Idempotent: running it twice is a no-op.
pub fn migrate(profile: &mut StartupProfile, persisted: &mut Vec<WorkspaceDef>) -> bool {
    if profile.schema_version >= CURRENT_SCHEMA_VERSION {
        return false;
    }
    persisted.retain(|w| {
        w.seed_key
            .as_deref()
            .map_or(true, |k| !RETIRED_SEED_KEYS.contains(&k))
    });
    // Ids of dropped workspaces must not linger in the ordering list.
    let live: HashSet<_> = persisted.iter().map(|w| w.id).collect();
    profile.workspace_order.retain(|id| live.contains(id));
    if profile
        .active_workspace
        .is_some_and(|id| !live.contains(&id))
    {
        profile.active_workspace = None;
    }
    profile.schema_version = CURRENT_SCHEMA_VERSION;
    true
}

/// Merge factory seed with the user's profile. User always wins.
///
/// 1. If `factory_seed_disabled` → return only persisted user workspaces.
/// 2. Otherwise start from factory workspaces for enabled modules.
/// 3. Drop any whose seed key is in `suppressed_factory`.
/// 4. Overlay persisted workspaces — a persisted workspace with the same
///    seed key REPLACES the factory version (user edits survive upgrades).
/// 5. Append user-created workspaces (no seed key, or unmatched).
/// 6. Sort by `seed_order` (stable).
/// 7. Order by `workspace_order`; unlisted go to the end in creation order.
pub fn resolve_startup(
    factory: Vec<WorkspaceDef>,
    persisted: Vec<WorkspaceDef>,
    profile: &StartupProfile,
) -> ShellState {
    let mut workspaces = Vec::new();

    if !profile.factory_seed_disabled {
        for mut factory_ws in factory {
            let Some(key) = factory_ws.seed_key.clone() else {
                workspaces.push(factory_ws);
                continue;
            };
            if profile.suppressed_factory.iter().any(|s| s == &key) {
                continue;
            }
            if let Some(persisted_ws) = persisted
                .iter()
                .find(|w| w.seed_key.as_deref() == Some(key.as_str()))
            {
                workspaces.push(persisted_ws.clone());
            } else {
                // Fresh factory seed — keep its id for ordering convenience.
                let _ = &mut factory_ws;
                workspaces.push(factory_ws);
            }
        }
    }

    // Append user-created / unmatched persisted workspaces.
    for ws in &persisted {
        let already = match &ws.seed_key {
            Some(key) => workspaces
                .iter()
                .any(|w| w.seed_key.as_deref() == Some(key.as_str())),
            None => workspaces.iter().any(|w| w.id == ws.id),
        };
        if !already {
            workspaces.push(ws.clone());
        }
    }

    // Deterministic factory tab order before the user's explicit order wins.
    workspaces.sort_by_key(|w| w.seed_order);

    // Order by profile.workspace_order; unlisted appended in current order.
    if !profile.workspace_order.is_empty() {
        let mut ordered = Vec::with_capacity(workspaces.len());
        let mut remaining = workspaces;
        for id in &profile.workspace_order {
            if let Some(pos) = remaining.iter().position(|w| w.id == *id) {
                ordered.push(remaining.remove(pos));
            }
        }
        ordered.append(&mut remaining);
        workspaces = ordered;
    }

    if workspaces.is_empty() {
        // Absolute fallback — empty shell is unusable.
        workspaces.push(crate::defaults::blank_workspace());
    }

    let active = profile
        .active_workspace
        .filter(|id| workspaces.iter().any(|w| w.id == *id))
        .or_else(|| workspaces.first().map(|w| w.id))
        .unwrap_or_else(WorkspaceId::new);

    ShellState {
        workspaces,
        active_workspace: active,
        status_message: "Ready".into(),
        pending_top_bar_actions: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::{PageDescriptor, PageNode};

    fn seeded(name: &str, key: &str) -> WorkspaceDef {
        WorkspaceDef::new(name, PageNode::leaf(PageDescriptor::new("core-ui", "view")))
            .with_seed_key(key)
    }

    fn user(name: &str) -> WorkspaceDef {
        let mut ws =
            WorkspaceDef::new(name, PageNode::leaf(PageDescriptor::new("core-ui", "view")));
        ws.user_modified = true;
        ws
    }

    #[test]
    fn empty_profile_returns_factory() {
        let factory = vec![seeded("Layout", "core/layout"), seeded("Analysis", "core/analysis")];
        let state = resolve_startup(factory.clone(), Vec::new(), &StartupProfile::default());
        assert_eq!(state.workspaces.len(), 2);
        assert_eq!(state.workspaces[0].name, "Layout");
        assert_eq!(state.active_workspace, state.workspaces[0].id);
    }

    #[test]
    fn suppressed_key_never_reappears() {
        let factory = vec![seeded("Layout", "core/layout"), seeded("Docs", "core/docs")];
        let profile = StartupProfile {
            suppressed_factory: vec!["core/docs".into()],
            ..Default::default()
        };
        let state = resolve_startup(factory, Vec::new(), &profile);
        assert_eq!(state.workspaces.len(), 1);
        assert_eq!(state.workspaces[0].name, "Layout");
    }

    #[test]
    fn persisted_replaces_same_seed_key() {
        let factory = vec![seeded("Layout", "core/layout")];
        let mut edited = seeded("My Layout", "core/layout");
        edited.user_modified = true;
        let state = resolve_startup(factory, vec![edited.clone()], &StartupProfile::default());
        assert_eq!(state.workspaces.len(), 1);
        assert_eq!(state.workspaces[0].name, "My Layout");
        assert_eq!(state.workspaces[0].id, edited.id);
    }

    #[test]
    fn factory_seed_disabled_skips_seeding() {
        let factory = vec![seeded("Layout", "core/layout")];
        let custom = user("Mine");
        let profile = StartupProfile {
            factory_seed_disabled: true,
            ..Default::default()
        };
        let state = resolve_startup(factory, vec![custom.clone()], &profile);
        assert_eq!(state.workspaces.len(), 1);
        assert_eq!(state.workspaces[0].id, custom.id);
    }

    #[test]
    fn workspace_order_honored() {
        let a = seeded("A", "a");
        let b = seeded("B", "b");
        let c = user("C");
        let order = vec![c.id, a.id];
        let profile = StartupProfile {
            workspace_order: order,
            active_workspace: Some(c.id),
            ..Default::default()
        };
        let state = resolve_startup(vec![a.clone(), b.clone()], vec![c.clone()], &profile);
        assert_eq!(
            state
                .workspaces
                .iter()
                .map(|w| w.name.as_str())
                .collect::<Vec<_>>(),
            vec!["C", "A", "B"]
        );
        assert_eq!(state.active_workspace, c.id);
    }

    #[test]
    fn seed_order_sorts_before_workspace_order() {
        let home = seeded("Home", "home/dashboard").with_seed_order(10);
        let pm = seeded("PM", "project-mgmt/main").with_seed_order(20);
        // Inserted in reverse registration order — seed_order still wins.
        let state = resolve_startup(vec![pm.clone(), home.clone()], Vec::new(), &StartupProfile::default());
        assert_eq!(
            state
                .workspaces
                .iter()
                .map(|w| w.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Home", "PM"]
        );
    }

    #[test]
    fn equal_seed_orders_keep_insertion_order() {
        let a = seeded("A", "a").with_seed_order(10);
        let b = seeded("B", "b").with_seed_order(10);
        let state = resolve_startup(vec![a, b], Vec::new(), &StartupProfile::default());
        assert_eq!(
            state
                .workspaces
                .iter()
                .map(|w| w.name.as_str())
                .collect::<Vec<_>>(),
            vec!["A", "B"]
        );
    }

    #[test]
    fn explicit_workspace_order_overrides_seed_order() {
        let home = seeded("Home", "home/dashboard").with_seed_order(10);
        let pm = seeded("PM", "project-mgmt/main").with_seed_order(20);
        let profile = StartupProfile {
            workspace_order: vec![pm.id, home.id],
            ..Default::default()
        };
        let state = resolve_startup(vec![home.clone(), pm.clone()], Vec::new(), &profile);
        assert_eq!(
            state
                .workspaces
                .iter()
                .map(|w| w.name.as_str())
                .collect::<Vec<_>>(),
            vec!["PM", "Home"]
        );
    }

    #[test]
    fn migrate_drops_retired_keys_once() {
        let layout = seeded("Layout", "core/layout");
        let analysis = seeded("Analysis", "core/analysis");
        let docs = seeded("Docs", "core/documentation");
        let template = seeded("Project", "project/template");
        let custom = user("Mine");
        let mut persisted = vec![layout.clone(), analysis, docs, template, custom.clone()];
        let mut profile = StartupProfile {
            schema_version: 0,
            workspace_order: vec![layout.id, custom.id],
            active_workspace: Some(layout.id),
            ..Default::default()
        };
        assert!(migrate(&mut profile, &mut persisted));
        assert_eq!(profile.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(persisted.len(), 1);
        assert_eq!(persisted[0].id, custom.id);
        assert_eq!(profile.workspace_order, vec![custom.id]);
        assert!(profile.active_workspace.is_none());
        assert!(!migrate(&mut profile, &mut persisted));
    }

    #[test]
    fn migrate_keeps_user_workspace_without_seed_key() {
        let custom = user("Mine");
        let mut persisted = vec![custom.clone()];
        let mut profile = StartupProfile {
            schema_version: 0,
            ..Default::default()
        };
        assert!(migrate(&mut profile, &mut persisted));
        assert_eq!(persisted.len(), 1);
        assert_eq!(persisted[0].id, custom.id);
    }
}
