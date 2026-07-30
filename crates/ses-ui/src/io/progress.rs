//! Progress bar — primary and optional secondary fraction layers.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ProgressTone {
    #[default]
    Neutral,
    Good,
    Warn,
    Over,
}

impl ProgressTone {
    pub fn class(self) -> &'static str {
        match self {
            Self::Neutral => "ses-progress-neutral",
            Self::Good => "ses-progress-good",
            Self::Warn => "ses-progress-warn",
            Self::Over => "ses-progress-over",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProgressDef {
    /// 0.0–1.0 derived fraction.
    pub fraction: f32,
    pub caption: Option<String>,
    pub secondary_fraction: Option<f32>,
    pub tone: ProgressTone,
}

fn clamp_fraction(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

fn width_pct(value: f32) -> String {
    format!("{:.2}%", clamp_fraction(value) * 100.0)
}

#[component]
pub fn ProgressBar(def: ProgressDef) -> Element {
    let tone = def.tone.class();
    let primary_width = width_pct(def.fraction);
    let secondary_width = def.secondary_fraction.map(width_pct);
    let caption = def.caption.clone();

    rsx! {
        div { class: "ses-progress {tone}",
            div { class: "ses-progress-track",
                if let Some(w) = secondary_width {
                    div {
                        class: "ses-progress-secondary",
                        style: "width: {w};",
                    }
                }
                div {
                    class: "ses-progress-primary",
                    style: "width: {primary_width};",
                }
            }
            if let Some(text) = caption {
                div { class: "ses-progress-caption", "{text}" }
            }
        }
    }
}
