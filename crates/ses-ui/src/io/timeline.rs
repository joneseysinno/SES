//! Timeline axis with dated milestone markers.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TimelineItem {
    pub id: String,
    pub title: String,
    pub at_utc: i64,
    pub done: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TimelineDef {
    pub items: Vec<TimelineItem>,
    pub range_start_utc: i64,
    pub range_end_utc: i64,
}

fn marker_left(at_utc: i64, start: i64, end: i64) -> f32 {
    if end <= start {
        return 50.0;
    }
    let span = (end - start) as f64;
    let pos = (at_utc - start) as f64;
    ((pos / span) * 100.0).clamp(0.0, 100.0) as f32
}

fn format_marker_time(at_utc: i64) -> String {
    // Scaffolding display — departments can replace with locale-aware formatting.
    format!("{at_utc}")
}

#[component]
pub fn Timeline(def: TimelineDef) -> Element {
    let start = def.range_start_utc;
    let end = def.range_end_utc;

    rsx! {
        div { class: "ses-timeline",
            div { class: "ses-timeline-axis" }
            for item in def.items.iter() {
                {
                    let left = marker_left(item.at_utc, start, end);
                    let left_pct = format!("{left:.2}%");
                    let dot_class = if item.done {
                        "ses-timeline-dot"
                    } else {
                        "ses-timeline-dot"
                    };
                    let title = item.title.clone();
                    let time_label = format_marker_time(item.at_utc);
                    rsx! {
                        div {
                            key: "{item.id}",
                            class: "ses-timeline-marker",
                            style: "left: {left_pct};",
                            div { class: "{dot_class}" }
                            span { class: "ses-timeline-label", "{title}" }
                            span { class: "ses-timeline-label", "{time_label}" }
                        }
                    }
                }
            }
        }
    }
}
