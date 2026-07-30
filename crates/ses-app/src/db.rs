//! Minimal infinite-db persistence for shell layout and startup profile.
//!
//! Spaces (scaffolding):
//! - `ui.workspaces` (id 1) — serialized workspace list (or legacy ShellState)
//! - `auth.users` (id 2) — placeholder for future permissions
//! - `ui.startup` (id 3) — StartupProfile
//!
//! Persistence is desktop-only (`feature = "desktop"`). Web uses in-memory defaults.

use ses_shell::{ShellState, StartupProfile, WorkspaceDef, default_shell};
use thiserror::Error;

pub const SPACE_UI_WORKSPACES: u64 = 1;
pub const SPACE_AUTH_USERS: u64 = 2;
pub const SPACE_UI_STARTUP: u64 = 3;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("engine: {0}")]
    Engine(String),
    #[error("codec: {0}")]
    Codec(String),
}

/// Load startup profile from disk, or default.
pub fn load_startup_profile() -> StartupProfile {
    #[cfg(feature = "desktop")]
    {
        match load_startup_from_db() {
            Ok(Some(p)) => p,
            Ok(None) => StartupProfile::default(),
            Err(e) => {
                eprintln!("ses: startup profile unavailable ({e}); using defaults");
                StartupProfile::default()
            }
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        StartupProfile::default()
    }
}

/// Persist startup profile (no-op without desktop feature).
pub fn save_startup_profile(profile: &StartupProfile) {
    #[cfg(feature = "desktop")]
    {
        if let Err(e) = save_startup_to_db(profile) {
            eprintln!("ses: failed to save startup profile: {e}");
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = profile;
    }
}

/// Load persisted workspaces only (not a full ShellState).
pub fn load_workspaces() -> Vec<WorkspaceDef> {
    #[cfg(feature = "desktop")]
    {
        match load_shell_from_db() {
            Ok(Some(state)) => state.workspaces,
            Ok(None) => Vec::new(),
            Err(e) => {
                eprintln!("ses: layout db unavailable ({e}); no persisted workspaces");
                Vec::new()
            }
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        Vec::new()
    }
}

/// Load shell state from disk, or seed defaults.
/// Prefer resolving via [`ses_shell::resolve_startup`] with a module registry
/// when department factory seeds are available.
pub fn load_or_default_shell() -> ShellState {
    #[cfg(feature = "desktop")]
    {
        match load_shell_from_db() {
            Ok(Some(state)) => {
                bump_ids_past(&state);
                state
            }
            Ok(None) => {
                let state = default_shell();
                if let Err(e) = save_shell_to_db(&state) {
                    eprintln!("ses: could not seed layout db: {e}");
                }
                state
            }
            Err(e) => {
                eprintln!("ses: layout db unavailable ({e}); using defaults");
                default_shell()
            }
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        default_shell()
    }
}

pub fn bump_ids_past(state: &ShellState) {
    use ses_shell::ids::reset_id_counter;
    let mut max_id = 1u64;
    for ws in &state.workspaces {
        max_id = max_id.max(ws.id.0);
        for leaf_id in ws.layout.leaf_ids() {
            max_id = max_id.max(leaf_id.0);
        }
    }
    reset_id_counter(max_id.saturating_add(1));
}

/// Persist shell layout (no-op without desktop feature).
pub fn save_shell(state: &ShellState) {
    #[cfg(feature = "desktop")]
    {
        if let Err(e) = save_shell_to_db(state) {
            eprintln!("ses: failed to save layout: {e}");
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = state;
    }
}

#[cfg(feature = "desktop")]
fn data_dir() -> std::path::PathBuf {
    let base = dirs_next_data();
    let path = base.join("ses").join("db");
    let _ = std::fs::create_dir_all(&path);
    path
}

#[cfg(feature = "desktop")]
fn dirs_next_data() -> std::path::PathBuf {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        return std::path::PathBuf::from(local);
    }
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home).join(".local").join("share");
    }
    std::env::temp_dir()
}

#[cfg(feature = "desktop")]
fn open_db() -> Result<infinite_db::InfiniteDb, DbError> {
    use infinite_db::InfiniteDb;
    use infinite_db::infinitedb_core::address::SpaceId;
    use infinite_db::infinitedb_core::space::SpaceConfig;

    let path = data_dir();
    let db = InfiniteDb::open(&path).map_err(|e| DbError::Engine(e.to_string()))?;

    let _ = db.register_space(SpaceConfig::new(
        SpaceId(SPACE_UI_WORKSPACES),
        "ui.workspaces",
        1,
    ));
    let _ = db.register_space(SpaceConfig::new(
        SpaceId(SPACE_AUTH_USERS),
        "auth.users",
        1,
    ));
    let _ = db.register_space(SpaceConfig::new(
        SpaceId(SPACE_UI_STARTUP),
        "ui.startup",
        1,
    ));

    Ok(db)
}

#[cfg(feature = "desktop")]
fn load_shell_from_db() -> Result<Option<ShellState>, DbError> {
    use infinite_db::infinitedb_core::address::SpaceId;

    let db = open_db()?;
    let rows = db
        .query(SpaceId(SPACE_UI_WORKSPACES), None)
        .map_err(|e| DbError::Engine(e.to_string()))?;

    for record in &rows {
        if record.address.point.coords.first().copied() == Some(0) {
            if let Ok(state) = decode_shell(&record.data) {
                return Ok(Some(state));
            }
        }
    }

    for record in &rows {
        if let Ok(state) = decode_shell(&record.data) {
            return Ok(Some(state));
        }
    }

    Ok(None)
}

#[cfg(feature = "desktop")]
fn save_shell_to_db(state: &ShellState) -> Result<(), DbError> {
    use infinite_db::infinitedb_core::address::{DimensionVector, SpaceId};

    let db = open_db()?;
    let bytes = encode_json(state)?;
    db.insert(
        SpaceId(SPACE_UI_WORKSPACES),
        DimensionVector::new(vec![0]),
        bytes,
    )
    .map_err(|e| DbError::Engine(e.to_string()))?;
    db.sync().map_err(|e| DbError::Engine(e.to_string()))?;

    let _ = db.insert(
        SpaceId(SPACE_AUTH_USERS),
        DimensionVector::new(vec![0]),
        b"dev".to_vec(),
    );
    let _ = db.sync();

    Ok(())
}

#[cfg(feature = "desktop")]
fn load_startup_from_db() -> Result<Option<StartupProfile>, DbError> {
    use infinite_db::infinitedb_core::address::SpaceId;

    let db = open_db()?;
    let rows = db
        .query(SpaceId(SPACE_UI_STARTUP), None)
        .map_err(|e| DbError::Engine(e.to_string()))?;

    for record in &rows {
        if record.address.point.coords.first().copied() == Some(0) {
            if let Ok(p) = decode_json::<StartupProfile>(&record.data) {
                return Ok(Some(p));
            }
        }
    }
    Ok(None)
}

#[cfg(feature = "desktop")]
fn save_startup_to_db(profile: &StartupProfile) -> Result<(), DbError> {
    use infinite_db::infinitedb_core::address::{DimensionVector, SpaceId};

    let db = open_db()?;
    let bytes = encode_json(profile)?;
    db.insert(
        SpaceId(SPACE_UI_STARTUP),
        DimensionVector::new(vec![0]),
        bytes,
    )
    .map_err(|e| DbError::Engine(e.to_string()))?;
    db.sync().map_err(|e| DbError::Engine(e.to_string()))?;
    Ok(())
}

#[cfg(feature = "desktop")]
fn encode_json<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, DbError> {
    serde_json::to_vec(value).map_err(|e| DbError::Codec(e.to_string()))
}

#[cfg(feature = "desktop")]
fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, DbError> {
    serde_json::from_slice(bytes).map_err(|e| DbError::Codec(e.to_string()))
}

#[cfg(feature = "desktop")]
fn decode_shell(bytes: &[u8]) -> Result<ShellState, DbError> {
    decode_json(bytes)
}
