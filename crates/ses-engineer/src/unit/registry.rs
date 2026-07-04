use super::UnitEntry;

/// In-memory unit registry. Populated from the `units` InfiniteDB space at runtime.
#[derive(Debug, Clone)]
pub struct UnitRegistry {
    pub(crate) entries: Vec<UnitEntry>,
}

mod entries;
mod ephemeral;
mod get;
mod get_by_symbol;
mod new;
mod register;
