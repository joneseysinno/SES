use ses_engineer::Dim;

/// Named fact from the pipeline facts registry (ses-code-pipeline §4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactDefinition {
    pub name: &'static str,
    pub dimension: Option<Dim>,
    pub source: &'static str,
}

pub const FACT_SDC: &str = "sdc";
pub const FACT_ELEMENT_KIND: &str = "element.kind";
pub const FACT_WALL_LW: &str = "wall.lw";
pub const FACT_WALL_HW: &str = "wall.hw";
pub const FACT_WALL_TW: &str = "wall.tw";
pub const FACT_WALL_HW_OVER_LW: &str = "wall.hw_over_lw";
pub const FACT_CONC_FC: &str = "conc.fc";
pub const FACT_CONC_LAMBDA: &str = "conc.lambda";
pub const FACT_STEEL_FY: &str = "steel.fy";
pub const FACT_DEM_PU: &str = "dem.Pu";
pub const FACT_DEM_VU: &str = "dem.Vu";
pub const FACT_DEM_MU: &str = "dem.Mu";
pub const FACT_DEM_DELTA_U: &str = "dem.delta_u";
pub const FACT_DEM_HSX: &str = "dem.hsx";

static FACT_CATALOG: &[FactDefinition] = &[
    FactDefinition {
        name: FACT_SDC,
        dimension: None,
        source: "project design basis",
    },
    FactDefinition {
        name: FACT_ELEMENT_KIND,
        dimension: None,
        source: "Element",
    },
    FactDefinition {
        name: FACT_WALL_LW,
        dimension: Some(Dim::LENGTH),
        source: "Element",
    },
    FactDefinition {
        name: FACT_WALL_HW,
        dimension: Some(Dim::LENGTH),
        source: "Element",
    },
    FactDefinition {
        name: FACT_WALL_TW,
        dimension: Some(Dim::LENGTH),
        source: "Element",
    },
    FactDefinition {
        name: FACT_WALL_HW_OVER_LW,
        dimension: Some(Dim::DIMENSIONLESS),
        source: "derived (procedure)",
    },
    FactDefinition {
        name: FACT_CONC_FC,
        dimension: Some(Dim::STRESS),
        source: "Material",
    },
    FactDefinition {
        name: FACT_CONC_LAMBDA,
        dimension: Some(Dim::DIMENSIONLESS),
        source: "Material",
    },
    FactDefinition {
        name: FACT_STEEL_FY,
        dimension: Some(Dim::STRESS),
        source: "Material",
    },
    FactDefinition {
        name: FACT_DEM_PU,
        dimension: Some(Dim::FORCE),
        source: "Demand",
    },
    FactDefinition {
        name: FACT_DEM_VU,
        dimension: Some(Dim::FORCE),
        source: "Demand",
    },
    FactDefinition {
        name: FACT_DEM_MU,
        dimension: Some(Dim::MOMENT),
        source: "Demand",
    },
    FactDefinition {
        name: FACT_DEM_DELTA_U,
        dimension: Some(Dim::LENGTH),
        source: "Demand",
    },
    FactDefinition {
        name: FACT_DEM_HSX,
        dimension: Some(Dim::LENGTH),
        source: "Demand",
    },
];

pub fn catalog() -> &'static [FactDefinition] {
    FACT_CATALOG
}
