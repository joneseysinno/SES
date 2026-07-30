//! Document reference list.

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct FileListItem {
    pub id: String,
    pub name: String,
    pub meta: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FileListDef {
    pub items: Vec<FileListItem>,
}

#[component]
pub fn FileList(def: FileListDef) -> Element {
    rsx! {
        div { class: "ses-file-list",
            for item in def.items.iter() {
                div { key: "{item.id}", class: "ses-file-list-item",
                    span { "{item.name}" }
                    if let Some(meta) = item.meta.clone() {
                        span { class: "ses-muted", "{meta}" }
                    }
                }
            }
        }
    }
}
