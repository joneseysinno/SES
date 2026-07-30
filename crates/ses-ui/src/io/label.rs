//! Static label output.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct LabelDef {
    pub text: String,
    #[allow(dead_code)]
    pub muted: bool,
}

impl LabelDef {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            muted: false,
        }
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

#[component]
pub fn Label(def: LabelDef) -> Element {
    let class = if def.muted { "ses-muted" } else { "" };
    rsx! {
        p { class: "{class}", "{def.text}" }
    }
}
