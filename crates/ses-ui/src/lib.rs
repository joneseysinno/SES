//! SES reusable Dioxus UI — Screen, Workspace, Page, Pod, I/O.

pub mod context;
pub mod io;
pub mod module_ui;
pub mod page;
pub mod pod;
pub mod screen;
pub mod theme;
pub mod workspace;

pub use context::{
    use_flow, use_modules, use_modules_ui, use_shell, use_startup, use_user, FlowCtx, ModulesCtx,
    ShellCtx, StartupCtx, UserCtx,
};
pub use io::{
    Badge, BadgeDef, BadgeTone, DataTable, DateDef, DateInput, EngineerInfo, EngineerInput,
    FieldMeta, FileList, FileListDef, FileListItem, InputContainer, IoComponent, Kanban,
    KanbanCardKind, KanbanColumn, KanbanDef, Label, LabelDef, Metric, MetricDef, MultiSelectDef,
    MultiSelectInput, NumberDef, NumericalInput, OutputContainer, ProgressBar, ProgressDef,
    ProgressTone, SelectDef, SelectInput, SelectOption, SpecificCardSubtask, SpecificCardTask,
    SpecificKanbanCard, SpecificKanbanCardDef, SummaryKanbanCard, SummaryKanbanCardDef,
    TableColumn, TableDef, TableRow, TextDef, TextInput, Timeline, TimelineDef, TimelineItem,
    render_io,
};
pub use module_ui::{ModuleUiRegistry, PageCtx, SesModuleUi};
pub use page::{page_pods, PodShell, PodStack, PAGE_PODS_VIEWPORT_PX};
pub use screen::Screen;

pub use ses_shell::{PodDescriptor, PodKind, PodLayout};
