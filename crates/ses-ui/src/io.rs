//! Input / output data-flow containers and field widgets.

pub mod badge;
pub mod binding_indicator;
pub mod date_input;
pub mod engineer_input;
pub mod field;
pub mod file_list;
pub mod input_container;
pub mod kanban;
pub mod label;
pub mod metric;
pub mod numerical_input;
pub mod output_container;
pub mod progress;
pub mod select;
pub mod table;
pub mod text_input;
pub mod timeline;

pub use badge::{Badge, BadgeDef, BadgeTone};
pub use binding_indicator::BindingIndicator;
pub use date_input::{DateDef, DateInput};
pub use engineer_input::EngineerInput;
pub use field::{EngineerInfo, FieldMeta};
pub use file_list::{FileList, FileListDef, FileListItem};
pub use input_container::InputContainer;
pub use kanban::{
    Kanban, KanbanCardKind, KanbanColumn, KanbanDef, SpecificCardSubtask, SpecificCardTask,
    SpecificKanbanCard, SpecificKanbanCardDef, SummaryKanbanCard, SummaryKanbanCardDef,
};
pub use label::{Label, LabelDef};
pub use metric::{Metric, MetricDef};
pub use numerical_input::NumericalInput;
pub use output_container::OutputContainer;
pub use progress::{ProgressBar, ProgressDef, ProgressTone};
pub use select::{MultiSelectDef, MultiSelectInput, SelectDef, SelectInput, SelectOption};
pub use table::{DataTable, TableColumn, TableDef, TableRow};
pub use text_input::TextInput;
pub use timeline::{Timeline, TimelineDef, TimelineItem};

use dioxus::prelude::*;

/// Text field definition for declarative IO rendering.
#[derive(Clone, PartialEq, Debug)]
pub struct TextDef {
    pub meta: FieldMeta,
    pub value: String,
}

/// Numeric field definition for declarative IO rendering.
#[derive(Clone, PartialEq, Debug)]
pub struct NumberDef {
    pub meta: FieldMeta,
    pub value: String,
}

/// Unified IO component enum — every department widget maps here.
#[derive(Clone, PartialEq, Debug)]
pub enum IoComponent {
    Text(TextDef),
    Number(NumberDef),
    Date(DateDef),
    Select(SelectDef),
    MultiSelect(MultiSelectDef),
    Label(LabelDef),
    Badge(BadgeDef),
    Progress(ProgressDef),
    Table(TableDef),
    Kanban(KanbanDef),
    Timeline(TimelineDef),
    Metric(MetricDef),
    FileList(FileListDef),
}

/// Render any IO component. Input handlers are no-ops — departments wire actions at a higher layer.
pub fn render_io(comp: &IoComponent) -> Element {
    match comp {
        IoComponent::Text(d) => rsx! {
            TextInput {
                id: d.meta.id.clone(),
                label: d.meta.label.clone(),
                value: d.value.clone(),
                units: d.meta.units.clone(),
                placeholder: d.meta.placeholder.clone(),
                disabled: d.meta.disabled,
                error: d.meta.error.clone(),
                oninput: move |_| {},
            }
        },
        IoComponent::Number(d) => rsx! {
            NumericalInput {
                id: d.meta.id.clone(),
                label: d.meta.label.clone(),
                value: d.value.clone(),
                units: d.meta.units.clone(),
                placeholder: d.meta.placeholder.clone(),
                disabled: d.meta.disabled,
                error: d.meta.error.clone(),
                oninput: move |_| {},
            }
        },
        IoComponent::Date(d) => rsx! {
            DateInput {
                meta: d.meta.clone(),
                value: d.value.clone(),
                oninput: move |_| {},
            }
        },
        IoComponent::Select(d) => rsx! {
            SelectInput {
                meta: d.meta.clone(),
                value: d.value.clone(),
                options: d.options.clone(),
                onchange: move |_| {},
            }
        },
        IoComponent::MultiSelect(d) => rsx! {
            MultiSelectInput {
                meta: d.meta.clone(),
                values: d.values.clone(),
                options: d.options.clone(),
                onchange: move |_| {},
            }
        },
        IoComponent::Label(d) => rsx! { Label { def: d.clone() } },
        IoComponent::Badge(d) => rsx! { Badge { def: d.clone() } },
        IoComponent::Progress(d) => rsx! { ProgressBar { def: d.clone() } },
        IoComponent::Table(d) => rsx! { DataTable { def: d.clone() } },
        IoComponent::Kanban(d) => rsx! { Kanban { def: d.clone() } },
        IoComponent::Timeline(d) => rsx! { Timeline { def: d.clone() } },
        IoComponent::Metric(d) => rsx! { Metric { def: d.clone() } },
        IoComponent::FileList(d) => rsx! { FileList { def: d.clone() } },
    }
}
