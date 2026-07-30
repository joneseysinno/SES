//! Status badge — tone-colored pill label.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BadgeTone {
    #[default]
    Neutral,
    Good,
    Warn,
    Over,
    Danger,
}

impl BadgeTone {
    pub fn class(self) -> &'static str {
        match self {
            Self::Neutral => "ses-badge-neutral",
            Self::Good => "ses-badge-good",
            Self::Warn => "ses-badge-warn",
            Self::Over => "ses-badge-over",
            Self::Danger => "ses-badge-danger",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BadgeDef {
    pub label: String,
    pub tone: BadgeTone,
}

#[component]
pub fn Badge(def: BadgeDef) -> Element {
    let tone = def.tone.class();
    let label = def.label.clone();
    rsx! {
        span { class: "ses-badge {tone}", "{label}" }
    }
}
