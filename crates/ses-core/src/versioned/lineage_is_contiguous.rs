/// Returns `true` when version bytes form a contiguous chain (each step is +1).
///
/// Used to validate lineage tables at startup and in tests (Vocabulary §1.2).
#[allow(clippy::arithmetic_side_effects)]
pub const fn lineage_is_contiguous(versions: &[u8]) -> bool {
    if versions.is_empty() {
        return false;
    }
    let mut i = 1;
    while i < versions.len() {
        if versions[i] != versions[i - 1].wrapping_add(1) {
            return false;
        }
        i += 1;
    }
    true
}
