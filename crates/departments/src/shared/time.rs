//! UTC day/week math — integer only, no `chrono`. Correct for the firm's
//! timezone, wrong for anyone far enough east that a late Sunday entry lands
//! in next week. See plan Consequences before reaching for a timezone crate.

use crate::shared::DateRange;

/// Days since the Unix epoch. Floor division: negative timestamps round down.
pub fn epoch_day(now_utc: i64) -> i64 {
    now_utc.div_euclid(86_400)
}

/// 0 = Monday .. 6 = Sunday. 1970-01-01 (day 0) was a Thursday, hence +3.
pub fn weekday_index(now_utc: i64) -> u8 {
    (epoch_day(now_utc) + 3).rem_euclid(7) as u8
}

/// Monday 00:00:00 UTC through the following Monday 00:00:00 UTC (exclusive end).
pub fn week_range_utc(now_utc: i64) -> DateRange {
    let start_day = epoch_day(now_utc) - weekday_index(now_utc) as i64;
    DateRange {
        start_utc: start_day * 86_400,
        end_utc: (start_day + 7) * 86_400,
    }
}

/// Current wall-clock time, seconds since the Unix epoch (UTC).
pub fn now_utc() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Parse an ISO `YYYY-MM-DD` date into seconds since the Unix epoch
/// (00:00:00 UTC that day). `None` for malformed or out-of-range input.
pub fn parse_date_utc(s: &str) -> Option<i64> {
    let mut parts = s.splitn(3, '-');
    let y: i64 = parts.next()?.parse().ok()?;
    let m: i64 = parts.next()?.parse().ok()?;
    let d: i64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let y2 = if m <= 2 { y - 1 } else { y };
    let era = y2.div_euclid(400);
    let yoe = y2.rem_euclid(400); // [0, 399]
    let mp = if m > 2 { m - 3 } else { m + 9 }; // [0, 11]
    let doy = (153 * mp + 2) / 5 + d - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400)
}

/// Proleptic Gregorian (year, month, day) for a day count since the epoch.
/// Howard Hinnant's `civil_from_days` — pure integer arithmetic, no `chrono`.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097); // [0, 146096]
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    let year = if month <= 2 { y + 1 } else { y };
    (year, month, day)
}

/// Proleptic Gregorian civil year containing `now_utc`.
pub fn year_utc(now_utc: i64) -> u16 {
    let (year, _, _) = civil_from_days(epoch_day(now_utc));
    year.clamp(0, u16::MAX as i64) as u16
}

/// Format seconds since the Unix epoch as an ISO `YYYY-MM-DD` date (UTC).
pub fn format_date_utc(now_utc: i64) -> String {
    let (y, m, d) = civil_from_days(epoch_day(now_utc));
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2024-01-01T00:00:00Z — a Monday.
    const MONDAY: i64 = 1_704_067_200;

    #[test]
    fn weekday_index_epoch_is_thursday() {
        assert_eq!(weekday_index(0), 3);
    }

    #[test]
    fn monday_maps_to_itself() {
        let range = week_range_utc(MONDAY);
        assert_eq!(range.start_utc, MONDAY);
    }

    #[test]
    fn sunday_evening_stays_in_same_week() {
        let sunday_2359 = MONDAY + 6 * 86_400 + 23 * 3_600 + 59 * 60;
        let range = week_range_utc(sunday_2359);
        assert_eq!(range.start_utc, MONDAY);
        assert_eq!(range.end_utc, MONDAY + 7 * 86_400);
    }

    #[test]
    fn range_spans_exactly_one_week() {
        let range = week_range_utc(MONDAY + 3 * 86_400);
        assert_eq!(range.end_utc - range.start_utc, 604_800);
    }

    #[test]
    fn negative_timestamps_floor_correctly() {
        // One second before epoch — 1969-12-31T23:59:59Z.
        let range = week_range_utc(-1);
        assert!(range.start_utc <= -1);
        assert_eq!(range.end_utc - range.start_utc, 604_800);
    }

    #[test]
    fn year_utc_matches_known_dates() {
        assert_eq!(year_utc(MONDAY), 2024); // 2024-01-01T00:00:00Z
        assert_eq!(year_utc(MONDAY - 1), 2023); // 2023-12-31T23:59:59Z
        assert_eq!(year_utc(0), 1970); // epoch
    }

    #[test]
    fn parse_date_utc_round_trips_known_date() {
        assert_eq!(parse_date_utc("2024-01-01"), Some(MONDAY));
        assert_eq!(parse_date_utc("1970-01-01"), Some(0));
    }

    #[test]
    fn parse_date_utc_rejects_malformed_input() {
        assert_eq!(parse_date_utc(""), None);
        assert_eq!(parse_date_utc("2024-13-01"), None);
        assert_eq!(parse_date_utc("2024-01-01-extra"), None);
        assert_eq!(parse_date_utc("not-a-date"), None);
    }

    #[test]
    fn format_date_utc_matches_known_dates() {
        assert_eq!(format_date_utc(MONDAY), "2024-01-01");
        assert_eq!(format_date_utc(0), "1970-01-01");
    }
}
