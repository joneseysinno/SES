//! Project number generation.

/// Next number for `year` given every existing project number.
/// Format `YYYY-NNN`. Malformed and other-year numbers are ignored.
/// Rolls to four digits past 999 rather than wrapping or panicking.
pub fn next_number<'a>(year: u16, existing: impl Iterator<Item = &'a str>) -> String {
    let prefix = format!("{year}-");
    let max = existing
        .filter_map(|s| s.strip_prefix(prefix.as_str()))
        .filter_map(|s| s.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    let next = max.saturating_add(1);
    if next > 999 {
        format!("{year}-{next:04}")
    } else {
        format!("{year}-{next:03}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_starts_at_one() {
        assert_eq!(next_number(2026, std::iter::empty()), "2026-001");
    }

    #[test]
    fn gaps_are_not_filled() {
        let existing = ["2026-001", "2026-005"];
        assert_eq!(next_number(2026, existing.iter().copied()), "2026-006");
    }

    #[test]
    fn other_year_and_malformed_ignored() {
        let existing = ["2025-999", "2026-abc", "not-a-number"];
        assert_eq!(next_number(2026, existing.iter().copied()), "2026-001");
    }

    #[test]
    fn rolls_to_four_digits_past_999() {
        let existing = ["2026-999"];
        assert_eq!(next_number(2026, existing.iter().copied()), "2026-1000");
    }
}
