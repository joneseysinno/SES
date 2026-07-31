use crate::project::bridge::ProjectQuery;
use crate::shared::ProjectId;
use crate::store::{use_dept_store, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, FileList, FileListDef, FileListItem, Label, LabelDef, PageCtx, PodDescriptor,
    PodKind,
};

#[component]
pub fn ProjectDocsPage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw);

    let pods = vec![PodDescriptor::stable(1, PodKind::Scroller, "Documents")];

    let body = match project_id {
        Some(project_id) => {
            let docs = match store.read().query_project(ProjectQuery::ListDocs(project_id)) {
                Ok(ProjectQueryResult::Docs(d)) => d,
                _ => vec![],
            };
            if docs.is_empty() {
                // Scaffold placeholders until authored docs exist for the project.
                rsx! {
                    FileList {
                        def: FileListDef {
                            items: vec![
                                FileListItem {
                                    id: "placeholder-1".into(),
                                    name: "Calc Package Rev A".into(),
                                    meta: Some("Calc".into()),
                                },
                                FileListItem {
                                    id: "placeholder-2".into(),
                                    name: "Structural Drawings S1.0".into(),
                                    meta: Some("Drawing".into()),
                                },
                                FileListItem {
                                    id: "placeholder-3".into(),
                                    name: "Geotech Report".into(),
                                    meta: Some("Report".into()),
                                },
                            ],
                        },
                    }
                }
            } else {
                rsx! {
                    FileList {
                        def: FileListDef {
                            items: docs
                                .into_iter()
                                .map(|d| FileListItem {
                                    id: d.id.0.to_string(),
                                    name: d.title,
                                    meta: Some(format!("{:?} · {}", d.category, d.revision)),
                                })
                                .collect(),
                        },
                    }
                }
            }
        }
        None => rsx! {
            Label {
                def: LabelDef::new("Bind a project_id to open documents.").muted(),
            }
        },
    };

    rsx! {
        div { class: "ses-page ses-page-project-docs",
            {page_pods(pods, ctx.pod_layout.clone(), vec![(1, body)])}
        }
    }
}
