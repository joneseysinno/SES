use crate::project::bridge::ProjectQuery;
use crate::shared::ProjectId;
use crate::store::{use_dept_store, ProjectQueryResult};
use dioxus::prelude::*;
use ses_ui::{
    page_pods, Label, LabelDef, PageCtx, PodDescriptor, PodKind, Timeline, TimelineDef,
    TimelineItem,
};

#[component]
pub fn ProjectTimelinePage(ctx: PageCtx) -> Element {
    let store = use_dept_store();
    let project_id = ctx
        .binding_get("project_id")
        .and_then(|s| s.parse::<u64>().ok())
        .map(ProjectId::from_raw);

    let pods = vec![PodDescriptor::stable(1, PodKind::Scroller, "Timeline")];

    let body = match project_id {
        Some(project_id) => {
            let milestones =
                match store
                    .read()
                    .query_project(ProjectQuery::ListMilestones(project_id))
                {
                    Ok(ProjectQueryResult::Milestones(m)) => m,
                    _ => vec![],
                };
            if milestones.is_empty() {
                rsx! {
                    Label {
                        def: LabelDef::new("No milestones yet.").muted(),
                    }
                }
            } else {
                let range_start = milestones.iter().map(|m| m.target_utc).min().unwrap_or(0);
                let range_end = milestones.iter().map(|m| m.target_utc).max().unwrap_or(0);
                rsx! {
                    Timeline {
                        def: TimelineDef {
                            items: milestones
                                .into_iter()
                                .map(|m| TimelineItem {
                                    id: m.id.0.to_string(),
                                    title: m.title,
                                    at_utc: m.target_utc,
                                    done: m.actual_utc.is_some(),
                                })
                                .collect(),
                            range_start_utc: range_start,
                            range_end_utc: range_end.max(range_start),
                        },
                    }
                }
            }
        }
        None => rsx! {
            Label {
                def: LabelDef::new("Bind a project_id to open the timeline.").muted(),
            }
        },
    };

    rsx! {
        div { class: "ses-page ses-page-project-timeline",
            {page_pods(pods, ctx.pod_layout.clone(), vec![(1, body)])}
        }
    }
}
