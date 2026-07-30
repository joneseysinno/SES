//! Kanban board — columns of cards with click-to-move scaffolding.

use crate::io::badge::{Badge, BadgeDef};
use crate::io::metric::{Metric, MetricDef};
use crate::io::progress::{ProgressBar, ProgressDef};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct KanbanColumn {
    pub id: String,
    pub title: String,
    pub accent: Option<String>,
    pub limit: Option<u16>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct KanbanCard {
    pub id: String,
    pub column_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub badges: Vec<BadgeDef>,
    pub progress: Option<ProgressDef>,
    pub metrics: Vec<MetricDef>,
    pub order: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct KanbanDef {
    pub columns: Vec<KanbanColumn>,
    pub cards: Vec<KanbanCard>,
    /// ActionId as string for scaffolding.
    pub on_move: Option<String>,
    pub on_open: Option<String>,
    pub allow_add: bool,
}

#[component]
pub fn Kanban(
    def: KanbanDef,
    #[props(default)]
    on_move: EventHandler<(String, String)>,
    #[props(default)]
    on_open: EventHandler<String>,
) -> Element {
    let mut selected = use_signal(|| None::<String>);
    let mut cards = use_signal(|| def.cards.clone());

    let move_action = def.on_move.clone();
    let _open_action = def.on_open.clone();

    let cards_by_column = |column_id: &str| -> Vec<KanbanCard> {
        let mut col_cards: Vec<KanbanCard> = cards()
            .into_iter()
            .filter(|c| c.column_id == column_id)
            .collect();
        col_cards.sort_by_key(|c| c.order);
        col_cards
    };

    rsx! {
        div { class: "ses-kanban",
            for col in def.columns.iter() {
                {
                    let col_id = col.id.clone();
                    let col_cards = cards_by_column(&col_id);
                    let count = col_cards.len();
                    let wip_warn = col.limit.is_some_and(|limit| count > limit as usize);
                    let header_class = if wip_warn {
                        "ses-kanban-col-header ses-wip-warn"
                    } else {
                        "ses-kanban-col-header"
                    };
                    let accent_style = col.accent.as_ref().map(|a| format!("border-top: 3px solid {a};"));
                    let show_move = selected().is_some_and(|sid| {
                        cards().iter().any(|c| c.id == sid && c.column_id != col_id)
                    });
                    let selected_id = selected();
                    let move_action = move_action.clone();

                    rsx! {
                        div {
                            key: "{col.id}",
                            class: "ses-kanban-col",
                            style: accent_style.unwrap_or_default(),
                            div { class: "{header_class}",
                                span { "{col.title}" }
                                span { class: "ses-kanban-count", "{count}" }
                                if show_move {
                                    if let Some(card_id) = selected_id.clone() {
                                        button {
                                            class: "ses-ghost",
                                            r#type: "button",
                                            title: "Move selected card here",
                                            onclick: {
                                                let col_id = col_id.clone();
                                                let card_id = card_id.clone();
                                                move |_| {
                                                    cards.write().iter_mut().for_each(|c| {
                                                        if c.id == card_id {
                                                            c.column_id = col_id.clone();
                                                        }
                                                    });
                                                    selected.set(None);
                                                    on_move.call((card_id.clone(), col_id.clone()));
                                                    let _ = move_action.as_deref();
                                                }
                                            },
                                            "Move here"
                                        }
                                    }
                                }
                            }
                            div { class: "ses-kanban-col-body",
                                for card in col_cards {
                                    {
                                        let card_id = card.id.clone();
                                        let is_selected = selected() == Some(card_id.clone());
                                        let card_class = if is_selected {
                                            "ses-kanban-card ses-kanban-drop"
                                        } else {
                                            "ses-kanban-card"
                                        };
                                        let open_id = card_id.clone();

                                        rsx! {
                                            div {
                                                key: "{card.id}",
                                                class: "{card_class}",
                                                draggable: true,
                                                onclick: move |_| {
                                                    selected.set(Some(card_id.clone()));
                                                },
                                                ondoubleclick: move |_| {
                                                    on_open.call(open_id.clone());
                                                },
                                                div { class: "ses-kanban-card-title", "{card.title}" }
                                                if let Some(sub) = card.subtitle.clone() {
                                                    div { class: "ses-kanban-card-sub", "{sub}" }
                                                }
                                                if !card.badges.is_empty() {
                                                    div { class: "ses-kanban-card-badges",
                                                        for badge in card.badges.clone() {
                                                            Badge { def: badge }
                                                        }
                                                    }
                                                }
                                                if let Some(progress) = card.progress.clone() {
                                                    div { class: "ses-kanban-card-progress",
                                                        ProgressBar { def: progress }
                                                    }
                                                }
                                                if !card.metrics.is_empty() {
                                                    div { class: "ses-kanban-card-badges",
                                                        for metric in card.metrics.clone() {
                                                            Metric { def: metric }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if def.allow_add {
                div { class: "ses-kanban-col ses-kanban-unsorted",
                    div { class: "ses-kanban-col-header", "Add card" }
                    div { class: "ses-kanban-col-body",
                        p { class: "ses-muted", "Add-card action wiring is department-owned." }
                    }
                }
            }
        }
    }
}
