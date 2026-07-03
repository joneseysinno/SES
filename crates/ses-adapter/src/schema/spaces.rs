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

/// Spaces 1–6 use companion error spaces (InfiniteDB default).
pub const ERROR_SPACE_IDS: &[SpaceId] = &[
    SPACE_PROJECTS,
    SPACE_ELEMENTS,
    SPACE_MATERIALS,
    SPACE_DEMANDS,
    SPACE_ANALYSES,
    SPACE_CHECK_RESULTS,
];

/// Reference-data spaces that opt out of companion error spaces.
pub const WITHOUT_ERROR_SPACE_IDS: &[SpaceId] = &[SPACE_CODES, SPACE_PROVISIONS, SPACE_UNITS];

/// Provisions coordinate layout: `[code_seq, chapter, section, subsection, item, sub_item]`.
pub const PROVISION_COORD_LABELS: &[&str] = &[
    "code_seq",
    "chapter",
    "section",
    "subsection",
    "item",
    "sub_item",
];
