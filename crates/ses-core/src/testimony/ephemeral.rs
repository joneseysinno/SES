/// Marker for derived values that must not carry persisted testimony (Vocabulary §1.3).
///
/// Ephemeral types record which persisted pins they were derived from so
/// staleness can be detected without fabricating authorship.
pub trait Ephemeral {
    /// Witness labels naming the persisted inputs this value derives from.
    const DERIVED_FROM: &'static [&'static str];
}
