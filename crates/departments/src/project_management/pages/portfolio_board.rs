use crate::project::payloads::BoardConfig;
use crate::project::progress::compute;
use crate::project_management::bridge::mock_projects;
use crate::project_management::payloads::ProjectPhase;
use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn PortfolioBoardPage(ctx: PageCtx) -> Element {
    let projects = mock_projects();

    rsx! {
        div { class: "ses-page ses-page-portfolio-board",
            h2 { "Portfolio Board" }
            p { class: "ses-muted", "One card per project with rolled-up progress." }
            div { class: "ses-portfolio-columns",
                for phase in ProjectPhase::all() {
                    div { class: "ses-portfolio-col", key: "{phase.column_id()}",
                        h3 { "{phase.title()}" }
                        ul {
                            for p in projects.iter().filter(|p| p.phase == *phase) {
                                {
                                    let board = BoardConfig::factory(p.id);
                                    let progress = compute(&board, &[], &[]);
                                    let pct = (progress.fraction() * 100.0).round() as i32;
                                    rsx! {
                                        li { key: "{p.id.0}",
                                            "{p.name} ({p.number}) — {pct}%"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
