use crate::project_management::bridge::ProjectMgmtQuery;
use crate::store::{use_dept_store, MgmtQueryResult};
use dioxus::prelude::*;
use ses_ui::{page_pods, Metric, MetricDef, PageCtx, PodDescriptor, PodKind};

#[component]
pub fn ProjectMetricsPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let (active_count, contract_cents) = match store.read().query_mgmt(ProjectMgmtQuery::PortfolioMetrics)
    {
        Ok(MgmtQueryResult::PortfolioMetrics {
            active_count,
            contract_cents,
        }) => (active_count, contract_cents),
        _ => (0, 0),
    };
    let pipeline = format!(
        "${}.{:02}",
        contract_cents / 100,
        (contract_cents % 100).unsigned_abs()
    );

    let pods = vec![PodDescriptor::stable(1, PodKind::Summary, "Firm metrics")];

    rsx! {
        div { class: "ses-page ses-page-project-metrics",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![(
                    1,
                    rsx! {
                        Metric {
                            def: MetricDef {
                                label: "Active projects".into(),
                                value: active_count.to_string(),
                                delta: None,
                                delta_up: None,
                            },
                        }
                        Metric {
                            def: MetricDef {
                                label: "Pipeline value".into(),
                                value: pipeline,
                                delta: None,
                                delta_up: None,
                            },
                        }
                    },
                )],
            )}
        }
    }
}
