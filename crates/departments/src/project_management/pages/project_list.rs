use crate::project_management::bridge::mock_projects;
use dioxus::prelude::*;
use ses_ui::PageCtx;

#[component]
pub fn ProjectListPage(ctx: PageCtx) -> Element {
    let projects = mock_projects();
    rsx! {
        div { class: "ses-page ses-page-project-list",
            h2 { "Project List" }
            table { class: "ses-data-table",
                thead {
                    tr {
                        th { "Number" }
                        th { "Name" }
                        th { "Client" }
                        th { "Phase" }
                    }
                }
                tbody {
                    for p in projects {
                        tr { key: "{p.id.0}",
                            td { "{p.number}" }
                            td { "{p.name}" }
                            td { "{p.client.name}" }
                            td { "{p.phase.title()}" }
                        }
                    }
                }
            }
        }
    }
}
