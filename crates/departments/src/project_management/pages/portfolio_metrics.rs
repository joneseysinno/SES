use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn PortfolioMetricsPage(ctx: PageCtx) -> Element {
    rsx! {
        div { class: "ses-page ses-page-portfolio-metrics",
            h2 { "Portfolio Metrics" }
            p { class: "ses-muted", "Firm-level rollups — placeholder." }
            div { class: "ses-metrics-row",
                div { class: "ses-metric", "Active projects: 1" }
                div { class: "ses-metric", "Pipeline value: $250k" }
            }
        }
    }
}
