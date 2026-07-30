/// Counter names for persisted_counters (ses-vocabulary §4).
pub const COUNTER_PROJ: &str = "proj";
pub const COUNTER_EDGE: &str = "edge";

pub fn counter_elem(project_seq: u64) -> String {
    format!("elem:{project_seq}")
}

pub fn counter_matl(project_seq: u64) -> String {
    format!("matl:{project_seq}")
}

pub fn counter_combo(project_seq: u64, element_seq: u64) -> String {
    format!("combo:{project_seq}:{element_seq}")
}

pub fn counter_run(project_seq: u64, element_seq: u64) -> String {
    format!("run:{project_seq}:{element_seq}")
}
