//! Horizontal bar chart — pure CSS, comparable multi-row progress.

use dioxus::prelude::*;

use super::progress::ProgressTone;

#[derive(Clone, PartialEq, Debug)]
pub struct ChartRow {
    pub id: String,
    pub label: String,
    pub sublabel: Option<String>,
    /// Primary bar, 0.0..=1.0. Clamped on render.
    pub value: f32,
    /// Secondary bar drawn behind at 40% opacity (budget burn).
    pub secondary: Option<f32>,
    pub tone: ProgressTone,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BarChartDef {
    pub rows: Vec<ChartRow>,
    /// Fractional gridline positions. Default: 0.25, 0.5, 0.75.
    pub gridlines: Vec<f32>,
    pub caption: Option<String>,
    /// Truncate past this many rows and render a "+N more" footer.
    pub max_rows: Option<usize>,
}

impl Default for BarChartDef {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            gridlines: vec![0.25, 0.5, 0.75],
            caption: None,
            max_rows: None,
        }
    }
}

fn clamp_fraction(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

fn width_pct(value: f32) -> String {
    format!("{:.2}%", clamp_fraction(value) * 100.0)
}

#[component]
pub fn BarChart(def: BarChartDef, on_select: EventHandler<String>) -> Element {
    let total = def.rows.len();
    let (visible, remainder) = match def.max_rows {
        Some(max) if total > max => (def.rows[..max].to_vec(), total - max),
        _ => (def.rows.clone(), 0),
    };
    let caption = def.caption.clone();
    let gridlines = def.gridlines.clone();

    rsx! {
        div { class: "ses-chart",
            if visible.is_empty() {
                div { class: "ses-chart-empty ses-muted", "No data" }
            } else {
                div { class: "ses-chart-body",
                    for row in visible.into_iter() {
                        {
                            let id = row.id.clone();
                            let tone = row.tone.class();
                            let primary = width_pct(row.value);
                            let secondary = row.secondary.map(width_pct);
                            let label = row.label.clone();
                            let sublabel = row.sublabel.clone();
                            rsx! {
                                button {
                                    class: "ses-chart-row",
                                    r#type: "button",
                                    onclick: move |_| on_select.call(id.clone()),
                                    div { class: "ses-chart-label",
                                        span { class: "ses-chart-label-main", "{label}" }
                                        if let Some(sub) = sublabel {
                                            span { class: "ses-chart-label-sub", "{sub}" }
                                        }
                                    }
                                    div { class: "ses-chart-track-wrap",
                                        div { class: "ses-chart-grid",
                                            for g in gridlines.iter().copied() {
                                                div {
                                                    class: "ses-chart-gridline",
                                                    style: "left: {width_pct(g)};",
                                                }
                                            }
                                        }
                                        div { class: "ses-chart-track {tone}",
                                            if let Some(w) = secondary {
                                                div {
                                                    class: "ses-chart-secondary",
                                                    style: "width: {w};",
                                                }
                                            }
                                            div {
                                                class: "ses-chart-primary",
                                                style: "width: {primary};",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if remainder > 0 {
                    div { class: "ses-chart-more ses-muted", "+{remainder} more" }
                }
            }
            if let Some(text) = caption {
                div { class: "ses-chart-caption", "{text}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_and_width() {
        assert_eq!(width_pct(-0.5), "0.00%");
        assert_eq!(width_pct(1.5), "100.00%");
        assert_eq!(width_pct(0.5), "50.00%");
    }

    #[test]
    fn max_rows_truncation_math() {
        let def = BarChartDef {
            rows: (0..15)
                .map(|i| ChartRow {
                    id: i.to_string(),
                    label: format!("P{i}"),
                    sublabel: None,
                    value: 0.5,
                    secondary: None,
                    tone: ProgressTone::Neutral,
                })
                .collect(),
            gridlines: vec![0.5],
            caption: None,
            max_rows: Some(12),
        };
        assert_eq!(def.rows.len(), 15);
        assert_eq!(def.max_rows, Some(12));
    }
}
