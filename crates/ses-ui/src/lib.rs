//! SES reusable Dioxus UI — Screen, Workspace, Page, Pod, I/O.

pub mod context;
pub mod io;
pub mod module_ui;
pub mod page;
pub mod pod;
pub mod screen;
pub mod theme;
pub mod workspace;

pub use context::{FlowCtx, ModulesCtx, ShellCtx, StartupCtx, UserCtx};
pub use io::{
    Badge, BadgeDef, BadgeTone, DataTable, DateDef, DateInput, EngineerInfo, EngineerInput,
    FieldMeta, FileList, FileListDef, FileListItem, InputContainer, IoComponent, Kanban,
    KanbanCard, KanbanColumn, KanbanDef, Label, LabelDef, Metric, MetricDef, MultiSelectDef,
    MultiSelectInput, NumberDef, NumericalInput, OutputContainer, ProgressBar, ProgressDef,
    ProgressTone, SelectDef, SelectInput, SelectOption, TableColumn, TableDef, TableRow, TextDef,
    TextInput, Timeline, TimelineDef, TimelineItem, render_io,
};
pub use module_ui::{ModuleUiRegistry, PageCtx, SesModuleUi};
pub use screen::Screen;
