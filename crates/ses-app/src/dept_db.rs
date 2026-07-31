//! Domain infinite-db persistence for department payloads.
//!
//! Uses a **separate** DB root from UI shell spaces (`ses/db/domain`) so
//! adapter SpaceIds 1–16 never collide with UI spaces 1–3.

use departments::{
    project::payloads::{BoardCard, BoardConfig, DocRef, Milestone, Task, TimeEntry},
    project_management::payloads::{ProjectRecord, Proposal},
    DeptStore,
};
use ses_adapter::codec::{decode_payload, encode_payload};
use ses_adapter::schema::{
    ALL_SPACES, SPACE_BOARD_CARDS, SPACE_BOARD_CONFIGS, SPACE_DOC_REFS, SPACE_MILESTONES,
    SPACE_PROJECTS, SPACE_PROPOSALS, SPACE_TASKS, SPACE_TIME_ENTRIES,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeptDbError {
    #[error("engine: {0}")]
    Engine(String),
    #[error("codec: {0}")]
    Codec(String),
}

/// Load department store from disk, or seed when empty.
pub fn load_or_seed_dept_store() -> DeptStore {
    #[cfg(feature = "desktop")]
    {
        match load_dept_store() {
            Ok(store) if !store.projects.is_empty() => {
                store.bump_ids();
                store
            }
            Ok(_) => {
                #[cfg(feature = "demo-seed")]
                {
                    let store = DeptStore::with_demo_seed();
                    save_dept_store(&store);
                    store
                }
                #[cfg(not(feature = "demo-seed"))]
                {
                    DeptStore::empty()
                }
            }
            Err(e) => {
                eprintln!("ses: department db unavailable ({e}); using in-memory store");
                #[cfg(feature = "demo-seed")]
                {
                    DeptStore::with_demo_seed()
                }
                #[cfg(not(feature = "demo-seed"))]
                {
                    DeptStore::empty()
                }
            }
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        #[cfg(feature = "demo-seed")]
        {
            DeptStore::with_demo_seed()
        }
        #[cfg(not(feature = "demo-seed"))]
        {
            DeptStore::empty()
        }
    }
}

/// Persist the full department working set.
pub fn save_dept_store(store: &DeptStore) {
    #[cfg(feature = "desktop")]
    {
        if let Err(e) = save_dept_store_inner(store) {
            eprintln!("ses: failed to save department store: {e}");
        }
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = store;
    }
}

#[cfg(feature = "desktop")]
fn domain_dir() -> std::path::PathBuf {
    let base = crate::db::data_dir_base();
    let path = base.join("ses").join("db").join("domain");
    let _ = std::fs::create_dir_all(&path);
    path
}

#[cfg(feature = "desktop")]
fn open_domain_db() -> Result<infinite_db::InfiniteDb, DeptDbError> {
    use infinite_db::InfiniteDb;
    use infinite_db::infinitedb_core::space::SpaceConfig;

    let db = InfiniteDb::open(domain_dir()).map_err(|e| DeptDbError::Engine(e.to_string()))?;
    for (id, name, dims) in ALL_SPACES {
        let _ = db.register_space(SpaceConfig::new(*id, *name, usize::from(*dims)));
    }
    Ok(db)
}

#[cfg(feature = "desktop")]
fn load_dept_store() -> Result<DeptStore, DeptDbError> {
    let db = open_domain_db()?;
    let mut store = DeptStore::new();

    for record in db
        .query(SPACE_PROJECTS, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<ProjectRecord>(&record.data) {
            Ok(p) => {
                store.projects.insert(p.id, p);
            }
            Err(e) => eprintln!("ses: skip project row: {e}"),
        }
    }

    for record in db
        .query(SPACE_BOARD_CONFIGS, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<BoardConfig>(&record.data) {
            Ok(b) => {
                store.boards.insert(b.project_id, b);
            }
            Err(e) => eprintln!("ses: skip board row: {e}"),
        }
    }

    for record in db
        .query(SPACE_BOARD_CARDS, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<BoardCard>(&record.data) {
            Ok(c) => {
                store.board_cards.insert(c.id, c);
            }
            Err(e) => eprintln!("ses: skip board card: {e}"),
        }
    }

    for record in db
        .query(SPACE_TASKS, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<Task>(&record.data) {
            Ok(t) => {
                store.tasks.insert(t.id, t);
            }
            Err(e) => eprintln!("ses: skip task row: {e}"),
        }
    }

    for record in db
        .query(SPACE_TIME_ENTRIES, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<TimeEntry>(&record.data) {
            Ok(e) => {
                store.time_entries.insert(e.id, e);
            }
            Err(e) => eprintln!("ses: skip time entry: {e}"),
        }
    }

    for record in db
        .query(SPACE_DOC_REFS, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<DocRef>(&record.data) {
            Ok(d) => {
                store.docs.insert(d.id, d);
            }
            Err(e) => eprintln!("ses: skip doc ref: {e}"),
        }
    }

    for record in db
        .query(SPACE_MILESTONES, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<Milestone>(&record.data) {
            Ok(m) => {
                store.milestones.insert(m.id, m);
            }
            Err(e) => eprintln!("ses: skip milestone: {e}"),
        }
    }

    for record in db
        .query(SPACE_PROPOSALS, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?
    {
        if record.tombstone {
            continue;
        }
        match decode_payload::<Proposal>(&record.data) {
            Ok(p) => {
                store.proposals.insert(p.id, p);
            }
            Err(e) => eprintln!("ses: skip proposal: {e}"),
        }
    }

    // Ensure every project has a board (repair incomplete seeds).
    let missing: Vec<_> = store
        .projects
        .keys()
        .copied()
        .filter(|id| !store.boards.contains_key(id))
        .collect();
    for id in missing {
        let board = BoardConfig::factory(id);
        store.boards.insert(id, board);
    }

    Ok(store)
}

#[cfg(feature = "desktop")]
fn save_dept_store_inner(store: &DeptStore) -> Result<(), DeptDbError> {
    use infinite_db::infinitedb_core::address::DimensionVector;

    let db = open_domain_db()?;

    // Replace strategy: delete all live rows then rewrite. Fine for scaffolding
    // volumes; later can switch to per-entity upsert.
    clear_space(&db, SPACE_PROJECTS)?;
    clear_space(&db, SPACE_BOARD_CONFIGS)?;
    clear_space(&db, SPACE_BOARD_CARDS)?;
    clear_space(&db, SPACE_TASKS)?;
    clear_space(&db, SPACE_TIME_ENTRIES)?;
    clear_space(&db, SPACE_DOC_REFS)?;
    clear_space(&db, SPACE_MILESTONES)?;
    clear_space(&db, SPACE_PROPOSALS)?;

    for p in store.projects.values() {
        let bytes = encode_payload(p).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_PROJECTS,
            DimensionVector::new(vec![coord(p.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for b in store.boards.values() {
        let bytes = encode_payload(b).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_BOARD_CONFIGS,
            DimensionVector::new(vec![coord(b.project_id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for c in store.board_cards.values() {
        let bytes = encode_payload(c).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_BOARD_CARDS,
            DimensionVector::new(vec![coord(c.project_id.0), coord(c.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for t in store.tasks.values() {
        let bytes = encode_payload(t).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_TASKS,
            DimensionVector::new(vec![coord(t.project_id.0), coord(t.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for e in store.time_entries.values() {
        let bytes = encode_payload(e).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_TIME_ENTRIES,
            DimensionVector::new(vec![coord(e.project_id.0), coord(e.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for d in store.docs.values() {
        let bytes = encode_payload(d).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_DOC_REFS,
            DimensionVector::new(vec![coord(d.project_id.0), coord(d.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for m in store.milestones.values() {
        let bytes = encode_payload(m).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_MILESTONES,
            DimensionVector::new(vec![coord(m.project_id.0), coord(m.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    for p in store.proposals.values() {
        let bytes = encode_payload(p).map_err(|e| DeptDbError::Codec(e.to_string()))?;
        db.insert(
            SPACE_PROPOSALS,
            DimensionVector::new(vec![coord(p.project_id.0), coord(p.id.0)]),
            bytes,
        )
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    }

    db.sync()
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    Ok(())
}

#[cfg(feature = "desktop")]
fn clear_space(
    db: &infinite_db::InfiniteDb,
    space: infinite_db::infinitedb_core::address::SpaceId,
) -> Result<(), DeptDbError> {
    let rows = db
        .query(space, None)
        .map_err(|e| DeptDbError::Engine(e.to_string()))?;
    for record in rows {
        if record.tombstone {
            continue;
        }
        let _ = db.delete(space, record.address.point.clone());
    }
    Ok(())
}

fn coord(id: u64) -> u32 {
    u32::try_from(id).unwrap_or(u32::MAX)
}
