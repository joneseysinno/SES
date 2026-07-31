//! Firm-level rollups derived from [`ProjectProgress`]. Ephemeral — never
//! persisted, always recomputed from the same numbers the project pages show.

use crate::project::progress::{ProgressTone, ProjectProgress};
use crate::project_management::payloads::ProjectRecord;
use crate::shared::Minutes;

/// Firm-level rollup. Ephemeral, like everything derived from testimony.
#[derive(Debug, Clone, PartialEq)]
pub struct PortfolioProgress {
    /// Number of rows aggregated.
    pub project_count: u32,
    /// Same count — callers pass an already-filtered slice (see
    /// [`crate::project_management::bridge::ProjectFilter::active`]).
    pub active_count: u32,
    /// sum(completed_estimate) / sum(total_estimate). NOT the mean of fractions.
    pub weighted_fraction: f32,
    pub spent_billable: Minutes,
    pub spent_nonbillable: Minutes,
    /// Projects whose tone is not [`ProgressTone::Good`].
    pub at_risk_count: u32,
}

/// Estimate-weighted rollup across every row. A 4-hour project and a
/// 4,000-hour project are not equal votes — this never averages fractions.
pub fn aggregate<'a>(rows: impl Iterator<Item = &'a ProjectProgress>) -> PortfolioProgress {
    let mut project_count = 0u32;
    let mut total_estimate: u64 = 0;
    let mut completed_estimate: u64 = 0;
    let mut spent_billable: u32 = 0;
    let mut spent_nonbillable: u32 = 0;
    let mut at_risk_count = 0u32;

    for p in rows {
        project_count = project_count.saturating_add(1);
        total_estimate = total_estimate.saturating_add(u64::from(p.total_estimate.0));
        completed_estimate = completed_estimate.saturating_add(u64::from(p.completed_estimate.0));
        spent_billable = spent_billable.saturating_add(p.spent_billable.0);
        spent_nonbillable = spent_nonbillable.saturating_add(p.spent_nonbillable.0);
        if p.tone() != ProgressTone::Good {
            at_risk_count = at_risk_count.saturating_add(1);
        }
    }

    let weighted_fraction = if total_estimate == 0 {
        0.0
    } else {
        (completed_estimate as f32 / total_estimate as f32).clamp(0.0, 1.0)
    };

    PortfolioProgress {
        project_count,
        active_count: project_count,
        weighted_fraction,
        spent_billable: Minutes(spent_billable),
        spent_nonbillable: Minutes(spent_nonbillable),
        at_risk_count,
    }
}

fn is_exception(p: &ProjectProgress) -> bool {
    matches!(p.tone(), ProgressTone::Warn | ProgressTone::Over)
}

/// Risk ordering for the chart: exceptions first, then
/// (spent_fraction − completion_fraction) descending — the projects burning
/// budget faster than they are earning it float to the top. Stable: equal
/// deltas keep their input order.
pub fn risk_sort(rows: &mut [(ProjectRecord, ProjectProgress)]) {
    rows.sort_by(|(_, a), (_, b)| {
        let a_exception = is_exception(a);
        let b_exception = is_exception(b);
        match (a_exception, b_exception) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let a_delta = a.burn_fraction() - a.fraction();
                let b_delta = b.burn_fraction() - b.fraction();
                b_delta
                    .partial_cmp(&a_delta)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_management::payloads::ProjectPhase;
    use crate::shared::{Address, Client, ProjectId};
    use ses_adapter::payload::{DesignBasis, ProjectStatus, RiskCategory, UnitSystemPref};

    fn progress(total_hours: u32, completed_hours: u32) -> ProjectProgress {
        ProjectProgress {
            total_estimate: Minutes::from_hours(total_hours),
            completed_estimate: Minutes::from_hours(completed_hours),
            spent_billable: Minutes(0),
            spent_nonbillable: Minutes(0),
            total_task_count: 1,
            done_task_count: 0,
            open_task_count: 1,
            blocked_task_count: 0,
            orphan_task_count: 0,
        }
    }

    fn record(id: u64, number: &str) -> ProjectRecord {
        ProjectRecord {
            id: ProjectId::from_raw(id),
            name: format!("Project {id}"),
            number: number.into(),
            client: Client::from_name(""),
            address: Address::default(),
            status: ProjectStatus::Active,
            phase: ProjectPhase::InProgress,
            manager: String::new(),
            start_utc: 0,
            target_finish_utc: None,
            contract_value: None,
            design_basis: DesignBasis {
                code_stack: vec![],
                amendment_branch: None,
                display_units: UnitSystemPref::Imperial,
                sds_milli: 0,
                sd1_milli: 0,
                seismic_design_category: String::new(),
                risk_category: RiskCategory::Ii,
            },
            engineer_of_record: String::new(),
            created_utc: 0,
        }
    }

    #[test]
    fn empty_aggregate_is_zero() {
        let p = aggregate(std::iter::empty());
        assert_eq!(p.project_count, 0);
        assert_eq!(p.weighted_fraction, 0.0);
        assert_eq!(p.at_risk_count, 0);
    }

    #[test]
    fn weighted_not_mean_of_fractions() {
        let small = progress(4, 4); // 100% of a 4h project
        let big = progress(4000, 0); // 0% of a 4000h project
        let rows = vec![small, big];
        let p = aggregate(rows.iter());
        assert!((p.weighted_fraction - 0.001).abs() < 0.001);
        assert!((p.weighted_fraction - 0.5).abs() > 0.1);
    }

    #[test]
    fn all_zero_estimates_no_divide_by_zero() {
        let rows = vec![progress(0, 0), progress(0, 0)];
        let p = aggregate(rows.iter());
        assert_eq!(p.weighted_fraction, 0.0);
    }

    #[test]
    fn at_risk_counts_every_non_good_tone() {
        let mut blocked = progress(10, 0);
        blocked.blocked_task_count = 1;
        let good = progress(10, 10);
        let neutral = progress(10, 5);
        let rows = vec![blocked, good, neutral];
        let p = aggregate(rows.iter());
        assert_eq!(p.at_risk_count, 2);
    }

    #[test]
    fn risk_sort_exceptions_precede_all_others() {
        let mut exception = progress(10, 0);
        exception.blocked_task_count = 1;
        let calm = progress(10, 10);
        let mut rows = vec![
            (record(1, "2026-001"), calm),
            (record(2, "2026-002"), exception),
        ];
        risk_sort(&mut rows);
        assert_eq!(rows[0].0.number, "2026-002");
    }

    #[test]
    fn risk_sort_orders_by_burn_minus_completion_descending() {
        // Both stay Neutral (non-exception) but carry different burn deltas.
        let mut on_track = progress(10, 8); // done 0.8
        on_track.spent_billable = Minutes::from_hours(8); // burn 0.8, delta 0.0
        let mut ahead_of_delivery = progress(10, 2); // done 0.2
        ahead_of_delivery.spent_billable = Minutes::from_hours(3); // burn 0.3, delta 0.1
        let mut rows = vec![
            (record(1, "2026-001"), on_track),
            (record(2, "2026-002"), ahead_of_delivery),
        ];
        risk_sort(&mut rows);
        assert_eq!(rows[0].0.number, "2026-002");
    }

    #[test]
    fn risk_sort_equal_deltas_keep_input_order() {
        let a = progress(10, 5);
        let b = progress(20, 10);
        let mut rows = vec![(record(1, "2026-001"), a), (record(2, "2026-002"), b)];
        risk_sort(&mut rows);
        assert_eq!(rows[0].0.number, "2026-001");
        assert_eq!(rows[1].0.number, "2026-002");
    }
}
