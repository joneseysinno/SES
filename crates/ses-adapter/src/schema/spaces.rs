use infinite_db::infinitedb_core::address::SpaceId;

pub const SPACE_PROJECTS: SpaceId = SpaceId(1);
pub const SPACE_ELEMENTS: SpaceId = SpaceId(2);
pub const SPACE_MATERIALS: SpaceId = SpaceId(3);
pub const SPACE_DEMANDS: SpaceId = SpaceId(4);
pub const SPACE_ANALYSES: SpaceId = SpaceId(5);
pub const SPACE_CHECK_RESULTS: SpaceId = SpaceId(6);
pub const SPACE_CODES: SpaceId = SpaceId(7);
pub const SPACE_PROVISIONS: SpaceId = SpaceId(8);
pub const SPACE_RELATIONS: SpaceId = SpaceId(9);
pub const SPACE_UNITS: SpaceId = SpaceId(10);
/// Department payloads (management / project working set).
pub const SPACE_TASKS: SpaceId = SpaceId(11);
pub const SPACE_BOARD_CONFIGS: SpaceId = SpaceId(12);
pub const SPACE_TIME_ENTRIES: SpaceId = SpaceId(13);
pub const SPACE_DOC_REFS: SpaceId = SpaceId(14);
pub const SPACE_MILESTONES: SpaceId = SpaceId(15);
pub const SPACE_PROPOSALS: SpaceId = SpaceId(16);
pub const SPACE_BOARD_CARDS: SpaceId = SpaceId(17);

pub const SPACE_NAME_PROJECTS: &str = "projects";
pub const SPACE_NAME_ELEMENTS: &str = "elements";
pub const SPACE_NAME_MATERIALS: &str = "materials";
pub const SPACE_NAME_DEMANDS: &str = "demands";
pub const SPACE_NAME_ANALYSES: &str = "analyses";
pub const SPACE_NAME_CHECK_RESULTS: &str = "check_results";
pub const SPACE_NAME_CODES: &str = "codes";
pub const SPACE_NAME_PROVISIONS: &str = "provisions";
pub const SPACE_NAME_RELATIONS: &str = "relations";
pub const SPACE_NAME_UNITS: &str = "units";
pub const SPACE_NAME_TASKS: &str = "tasks";
pub const SPACE_NAME_BOARD_CONFIGS: &str = "board_configs";
pub const SPACE_NAME_TIME_ENTRIES: &str = "time_entries";
pub const SPACE_NAME_DOC_REFS: &str = "doc_refs";
pub const SPACE_NAME_MILESTONES: &str = "milestones";
pub const SPACE_NAME_PROPOSALS: &str = "proposals";
pub const SPACE_NAME_BOARD_CARDS: &str = "board_cards";

/// Snake_case space registration names (ses-vocabulary §4).
pub const ALL_SPACE_NAMES: &[&str] = &[
    SPACE_NAME_PROJECTS,
    SPACE_NAME_ELEMENTS,
    SPACE_NAME_MATERIALS,
    SPACE_NAME_DEMANDS,
    SPACE_NAME_ANALYSES,
    SPACE_NAME_CHECK_RESULTS,
    SPACE_NAME_CODES,
    SPACE_NAME_PROVISIONS,
    SPACE_NAME_RELATIONS,
    SPACE_NAME_UNITS,
    SPACE_NAME_TASKS,
    SPACE_NAME_BOARD_CONFIGS,
    SPACE_NAME_TIME_ENTRIES,
    SPACE_NAME_DOC_REFS,
    SPACE_NAME_MILESTONES,
    SPACE_NAME_PROPOSALS,
    SPACE_NAME_BOARD_CARDS,
];

pub const SPACE_PROJECTS_DIMS: u8 = 1;
pub const SPACE_ELEMENTS_DIMS: u8 = 2;
pub const SPACE_MATERIALS_DIMS: u8 = 2;
pub const SPACE_DEMANDS_DIMS: u8 = 3;
pub const SPACE_ANALYSES_DIMS: u8 = 3;
pub const SPACE_CHECK_RESULTS_DIMS: u8 = 4;
pub const SPACE_CODES_DIMS: u8 = 1;
pub const SPACE_PROVISIONS_DIMS: u8 = 6;
pub const SPACE_RELATIONS_DIMS: u8 = 2;
pub const SPACE_UNITS_DIMS: u8 = 1;
/// `[proj_seq, task_seq]`
pub const SPACE_TASKS_DIMS: u8 = 2;
/// `[proj_seq]` — one board per project
pub const SPACE_BOARD_CONFIGS_DIMS: u8 = 1;
/// `[proj_seq, entry_seq]`
pub const SPACE_TIME_ENTRIES_DIMS: u8 = 2;
/// `[proj_seq, doc_seq]`
pub const SPACE_DOC_REFS_DIMS: u8 = 2;
/// `[proj_seq, milestone_seq]`
pub const SPACE_MILESTONES_DIMS: u8 = 2;
/// `[proj_seq, proposal_seq]`
pub const SPACE_PROPOSALS_DIMS: u8 = 2;
/// `[proj_seq, card_seq]`
pub const SPACE_BOARD_CARDS_DIMS: u8 = 2;

/// Spaces 1–6 use companion error spaces (InfiniteDB default).
pub const ERROR_SPACE_IDS: &[SpaceId] = &[
    SPACE_PROJECTS,
    SPACE_ELEMENTS,
    SPACE_MATERIALS,
    SPACE_DEMANDS,
    SPACE_ANALYSES,
    SPACE_CHECK_RESULTS,
];

/// Reference-data / department spaces that opt out of companion error spaces.
pub const WITHOUT_ERROR_SPACE_IDS: &[SpaceId] = &[
    SPACE_CODES,
    SPACE_PROVISIONS,
    SPACE_UNITS,
    SPACE_TASKS,
    SPACE_BOARD_CONFIGS,
    SPACE_TIME_ENTRIES,
    SPACE_DOC_REFS,
    SPACE_MILESTONES,
    SPACE_PROPOSALS,
    SPACE_BOARD_CARDS,
];

/// `(SpaceId, name, dims)` for every registered domain space.
pub const ALL_SPACES: &[(SpaceId, &str, u8)] = &[
    (SPACE_PROJECTS, SPACE_NAME_PROJECTS, SPACE_PROJECTS_DIMS),
    (SPACE_ELEMENTS, SPACE_NAME_ELEMENTS, SPACE_ELEMENTS_DIMS),
    (SPACE_MATERIALS, SPACE_NAME_MATERIALS, SPACE_MATERIALS_DIMS),
    (SPACE_DEMANDS, SPACE_NAME_DEMANDS, SPACE_DEMANDS_DIMS),
    (SPACE_ANALYSES, SPACE_NAME_ANALYSES, SPACE_ANALYSES_DIMS),
    (
        SPACE_CHECK_RESULTS,
        SPACE_NAME_CHECK_RESULTS,
        SPACE_CHECK_RESULTS_DIMS,
    ),
    (SPACE_CODES, SPACE_NAME_CODES, SPACE_CODES_DIMS),
    (SPACE_PROVISIONS, SPACE_NAME_PROVISIONS, SPACE_PROVISIONS_DIMS),
    (SPACE_RELATIONS, SPACE_NAME_RELATIONS, SPACE_RELATIONS_DIMS),
    (SPACE_UNITS, SPACE_NAME_UNITS, SPACE_UNITS_DIMS),
    (SPACE_TASKS, SPACE_NAME_TASKS, SPACE_TASKS_DIMS),
    (
        SPACE_BOARD_CONFIGS,
        SPACE_NAME_BOARD_CONFIGS,
        SPACE_BOARD_CONFIGS_DIMS,
    ),
    (
        SPACE_TIME_ENTRIES,
        SPACE_NAME_TIME_ENTRIES,
        SPACE_TIME_ENTRIES_DIMS,
    ),
    (SPACE_DOC_REFS, SPACE_NAME_DOC_REFS, SPACE_DOC_REFS_DIMS),
    (SPACE_MILESTONES, SPACE_NAME_MILESTONES, SPACE_MILESTONES_DIMS),
    (SPACE_PROPOSALS, SPACE_NAME_PROPOSALS, SPACE_PROPOSALS_DIMS),
    (
        SPACE_BOARD_CARDS,
        SPACE_NAME_BOARD_CARDS,
        SPACE_BOARD_CARDS_DIMS,
    ),
];

/// Provisions coordinate layout: `[code_seq, chapter, section, subsection, item, sub_item]`.
pub const PROVISION_COORD_LABELS: &[&str] = &[
    "code_seq",
    "chapter",
    "section",
    "subsection",
    "item",
    "sub_item",
];
