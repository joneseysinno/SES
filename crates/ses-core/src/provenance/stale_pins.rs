use super::{Pin, Revision};

/// Returns `true` when `pin` records an older revision than `current`.
pub fn pin_is_stale<Id>(pin: &Pin<Id>, current: Revision) -> bool {
    pin.at < current
}

/// Returns `true` when any pin is stale relative to `lookup`.
///
/// `lookup` must return the latest revision for each pinned id.
pub fn any_stale_pins<Id, F>(pins: &[Pin<Id>], lookup: F) -> bool
where
    Id: Eq,
    F: Fn(&Id) -> Revision,
{
    pins.iter().any(|pin| pin_is_stale(pin, lookup(&pin.id)))
}
