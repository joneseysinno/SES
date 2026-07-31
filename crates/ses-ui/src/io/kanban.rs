//! Kanban board — columns of summary / specific cards with click-to-move scaffolding.

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
pub struct SummaryKanbanCardDef {
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
pub struct SpecificCardSubtask {
    pub index: usize,
    pub label: String,
    pub done: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SpecificCardTask {
    pub id: String,
    pub title: String,
    pub done: bool,
    pub estimate: Option<String>,
    pub assignee: Option<String>,
    pub subtasks: Vec<SpecificCardSubtask>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SpecificKanbanCardDef {
    pub id: String,
    pub column_id: String,
    pub title: String,
    pub order: u32,
    pub tasks: Vec<SpecificCardTask>,
    pub allow_edit: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub enum KanbanCardKind {
    Summary(SummaryKanbanCardDef),
    Specific(SpecificKanbanCardDef),
}

impl KanbanCardKind {
    pub fn id(&self) -> &str {
        match self {
            Self::Summary(c) => &c.id,
            Self::Specific(c) => &c.id,
        }
    }

    pub fn column_id(&self) -> &str {
        match self {
            Self::Summary(c) => &c.column_id,
            Self::Specific(c) => &c.column_id,
        }
    }

    pub fn order(&self) -> u32 {
        match self {
            Self::Summary(c) => c.order,
            Self::Specific(c) => c.order,
        }
    }

    pub fn set_column_id(&mut self, column_id: String) {
        match self {
            Self::Summary(c) => c.column_id = column_id,
            Self::Specific(c) => c.column_id = column_id,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct KanbanDef {
    pub columns: Vec<KanbanColumn>,
    pub cards: Vec<KanbanCardKind>,
    /// ActionId as string for scaffolding.
    pub on_move: Option<String>,
    pub on_open: Option<String>,
    pub allow_add: bool,
    pub allow_add_column: bool,
}

#[component]
pub fn SummaryKanbanCard(
    def: SummaryKanbanCardDef,
    selected: bool,
    on_select: EventHandler<()>,
    on_open: EventHandler<()>,
) -> Element {
    let card_class = if selected {
        "ses-kanban-card ses-kanban-card-summary ses-kanban-drop"
    } else {
        "ses-kanban-card ses-kanban-card-summary"
    };

    rsx! {
        div {
            class: "{card_class}",
            draggable: true,
            onclick: move |_| on_select.call(()),
            ondoubleclick: move |_| on_open.call(()),
            div { class: "ses-kanban-card-title", "{def.title}" }
            if let Some(sub) = def.subtitle.clone() {
                div { class: "ses-kanban-card-sub", "{sub}" }
            }
            if !def.badges.is_empty() {
                div { class: "ses-kanban-card-badges",
                    for badge in def.badges.clone() {
                        Badge { def: badge }
                    }
                }
            }
            if let Some(progress) = def.progress.clone() {
                div { class: "ses-kanban-card-progress",
                    ProgressBar { def: progress }
                }
            }
            if !def.metrics.is_empty() {
                div { class: "ses-kanban-card-badges",
                    for metric in def.metrics.clone() {
                        Metric { def: metric }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SpecificKanbanCard(
    def: SpecificKanbanCardDef,
    selected: bool,
    on_select: EventHandler<()>,
    on_open: EventHandler<()>,
    #[props(default)]
    on_toggle_task: EventHandler<String>,
    #[props(default)]
    on_toggle_subtask: EventHandler<(String, usize)>,
    #[props(default)]
    on_add_task: EventHandler<String>,
    #[props(default)]
    on_add_subtask: EventHandler<String>,
) -> Element {
    let card_class = if selected {
        "ses-kanban-card ses-kanban-card-specific ses-kanban-drop"
    } else {
        "ses-kanban-card ses-kanban-card-specific"
    };
    let card_id = def.id.clone();
    let allow_edit = def.allow_edit;

    rsx! {
        div {
            class: "{card_class}",
            draggable: true,
            onclick: move |_| on_select.call(()),
            ondoubleclick: move |_| on_open.call(()),
            div { class: "ses-kanban-card-title", "{def.title}" }
            ul { class: "ses-kanban-task-list",
                for task in def.tasks.clone() {
                    {
                        let task_id = task.id.clone();
                        let task_id_sub = task.id.clone();
                        let task_id_add = task.id.clone();
                        rsx! {
                            li {
                                key: "{task.id}",
                                class: if task.done { "ses-kanban-task ses-kanban-task-done" } else { "ses-kanban-task" },
                                label { class: "ses-kanban-task-row",
                                    input {
                                        r#type: "checkbox",
                                        checked: task.done,
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            on_toggle_task.call(task_id.clone());
                                        },
                                    }
                                    span { "{task.title}" }
                                    if let Some(est) = task.estimate.clone() {
                                        span { class: "ses-kanban-task-meta", "{est}" }
                                    }
                                    if let Some(assignee) = task.assignee.clone() {
                                        span { class: "ses-kanban-task-meta", "{assignee}" }
                                    }
                                }
                                if !task.subtasks.is_empty() {
                                    ul { class: "ses-kanban-subtask-list",
                                        for sub in task.subtasks.clone() {
                                            {
                                                let idx = sub.index;
                                                let tid = task_id_sub.clone();
                                                rsx! {
                                                    li {
                                                        key: "{idx}",
                                                        class: if sub.done { "ses-kanban-subtask ses-kanban-task-done" } else { "ses-kanban-subtask" },
                                                        label {
                                                            input {
                                                                r#type: "checkbox",
                                                                checked: sub.done,
                                                                onclick: move |e| {
                                                                    e.stop_propagation();
                                                                    on_toggle_subtask.call((tid.clone(), idx));
                                                                },
                                                            }
                                                            span { "{sub.label}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if allow_edit {
                                    button {
                                        class: "ses-ghost ses-kanban-mini-btn",
                                        r#type: "button",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            on_add_subtask.call(task_id_add.clone());
                                        },
                                        "+ subtask"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if allow_edit {
                button {
                    class: "ses-ghost ses-kanban-mini-btn",
                    r#type: "button",
                    onclick: {
                        let card_id = card_id.clone();
                        move |e| {
                            e.stop_propagation();
                            on_add_task.call(card_id.clone());
                        }
                    },
                    "+ task"
                }
            }
        }
    }
}

#[component]
pub fn Kanban(
    def: KanbanDef,
    #[props(default)]
    on_move: EventHandler<(String, String)>,
    #[props(default)]
    on_open: EventHandler<String>,
    #[props(default)]
    on_add_column: EventHandler<KanbanColumn>,
    #[props(default)]
    on_toggle_task: EventHandler<(String, String)>,
    #[props(default)]
    on_toggle_subtask: EventHandler<(String, String, usize)>,
    #[props(default)]
    on_add_task: EventHandler<String>,
    #[props(default)]
    on_add_subtask: EventHandler<(String, String)>,
) -> Element {
    let mut selected = use_signal(|| None::<String>);
    let mut cards = use_signal(|| def.cards.clone());
    let mut columns = use_signal(|| def.columns.clone());
    let mut new_col_title = use_signal(String::new);
    let mut show_add_col = use_signal(|| false);

    // Keep local state aligned when parent rebuilds the def.
    use_effect(move || {
        cards.set(def.cards.clone());
        columns.set(def.columns.clone());
    });

    let move_action = def.on_move.clone();
    let allow_add = def.allow_add;
    let allow_add_column = def.allow_add_column;

    rsx! {
        div { class: "ses-kanban",
            for col in columns() {
                {
                    let col_id = col.id.clone();
                    let mut col_cards: Vec<KanbanCardKind> = cards()
                        .into_iter()
                        .filter(|c| c.column_id() == col_id)
                        .collect();
                    col_cards.sort_by_key(|c| c.order());
                    let count = col_cards.len();
                    let wip_warn = col.limit.is_some_and(|limit| count > limit as usize);
                    let header_class = if wip_warn {
                        "ses-kanban-col-header ses-wip-warn"
                    } else {
                        "ses-kanban-col-header"
                    };
                    let accent_style = col.accent.as_ref().map(|a| format!("border-top: 3px solid {a};"));
                    let show_move = selected().is_some_and(|sid| {
                        cards().iter().any(|c| c.id() == sid && c.column_id() != col_id)
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
                                                        if c.id() == card_id {
                                                            c.set_column_id(col_id.clone());
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
                                        let card_id = card.id().to_string();
                                        let is_selected = selected() == Some(card_id.clone());
                                        let open_id = card_id.clone();
                                        match card {
                                            KanbanCardKind::Summary(summary) => rsx! {
                                                SummaryKanbanCard {
                                                    key: "{summary.id}",
                                                    def: summary,
                                                    selected: is_selected,
                                                    on_select: {
                                                        let card_id = card_id.clone();
                                                        move |_| selected.set(Some(card_id.clone()))
                                                    },
                                                    on_open: move |_| on_open.call(open_id.clone()),
                                                }
                                            },
                                            KanbanCardKind::Specific(specific) => {
                                                let card_id_for_task = card_id.clone();
                                                let card_id_for_sub = card_id.clone();
                                                let card_id_for_add_task = card_id.clone();
                                                let card_id_for_add_sub = card_id.clone();
                                                rsx! {
                                                    SpecificKanbanCard {
                                                        key: "{specific.id}",
                                                        def: specific,
                                                        selected: is_selected,
                                                        on_select: {
                                                            let card_id = card_id.clone();
                                                            move |_| selected.set(Some(card_id.clone()))
                                                        },
                                                        on_open: move |_| on_open.call(open_id.clone()),
                                                        on_toggle_task: {
                                                            let card_id = card_id_for_task;
                                                            move |task_id| {
                                                                on_toggle_task.call((card_id.clone(), task_id));
                                                            }
                                                        },
                                                        on_toggle_subtask: {
                                                            let card_id = card_id_for_sub;
                                                            move |(task_id, idx)| {
                                                                on_toggle_subtask.call((card_id.clone(), task_id, idx));
                                                            }
                                                        },
                                                        on_add_task: {
                                                            let card_id = card_id_for_add_task;
                                                            move |_| on_add_task.call(card_id.clone())
                                                        },
                                                        on_add_subtask: {
                                                            let card_id = card_id_for_add_sub;
                                                            move |task_id| {
                                                                on_add_subtask.call((card_id.clone(), task_id));
                                                            }
                                                        },
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
            if allow_add_column {
                div { class: "ses-kanban-col ses-kanban-add-col",
                    div { class: "ses-kanban-col-header", "Columns" }
                    div { class: "ses-kanban-col-body",
                        if show_add_col() {
                            input {
                                class: "ses-kanban-col-input",
                                r#type: "text",
                                placeholder: "Column title",
                                value: "{new_col_title}",
                                oninput: move |e| new_col_title.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        let title = new_col_title().trim().to_string();
                                        if !title.is_empty() {
                                            let id = title
                                                .to_lowercase()
                                                .chars()
                                                .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
                                                .collect::<String>();
                                            let col = KanbanColumn {
                                                id: id.clone(),
                                                title: title.clone(),
                                                accent: None,
                                                limit: None,
                                            };
                                            columns.write().push(col.clone());
                                            new_col_title.set(String::new());
                                            show_add_col.set(false);
                                            on_add_column.call(col);
                                        }
                                    }
                                },
                            }
                            button {
                                class: "ses-ghost",
                                r#type: "button",
                                onclick: move |_| show_add_col.set(false),
                                "Cancel"
                            }
                        } else {
                            button {
                                class: "ses-ghost ses-kanban-add-col-btn",
                                r#type: "button",
                                onclick: move |_| show_add_col.set(true),
                                "+ Add column"
                            }
                        }
                    }
                }
            }
            if allow_add {
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
