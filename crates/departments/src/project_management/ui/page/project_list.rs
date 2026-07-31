use crate::project_management::bridge::ProjectMgmtQuery;
use crate::store::{use_dept_store, MgmtQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, DataTable, PageCtx, PodDescriptor, PodKind, TableColumn, TableDef, TableRow,
};

#[component]
pub fn ProjectListPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let projects = match store.read().query_mgmt(ProjectMgmtQuery::ListAll {
        filter: Default::default(),
    }) {
        Ok(MgmtQueryResult::Projects(p)) => p,
        _ => vec![],
    };

    let def = TableDef {
        columns: vec![
            TableColumn {
                id: "number".into(),
                title: "Number".into(),
                sortable: true,
            },
            TableColumn {
                id: "name".into(),
                title: "Name".into(),
                sortable: true,
            },
            TableColumn {
                id: "client".into(),
                title: "Client".into(),
                sortable: true,
            },
            TableColumn {
                id: "phase".into(),
                title: "Phase".into(),
                sortable: true,
            },
        ],
        rows: projects
            .iter()
            .map(|p| TableRow {
                id: p.id.0.to_string(),
                cells: vec![
                    p.number.clone(),
                    p.name.clone(),
                    p.client.name.clone(),
                    p.phase.title().into(),
                ],
            })
            .collect(),
    };

    let pods = vec![PodDescriptor::stable(1, PodKind::Anchor, "Projects")];

    rsx! {
        div { class: "ses-page ses-page-project-list",
            {page_pods(
                pods,
                ctx.pod_layout.clone(),
                vec![(1, rsx! { DataTable { def } })],
            )}
        }
    }
}
