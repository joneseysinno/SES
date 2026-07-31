//! Cross-department UI helpers.

#[cfg(feature = "project")]
use crate::project::progress::ProgressTone as DeptProgressTone;
#[cfg(feature = "project")]
use ses_ui::ProgressTone;

/// Map department progress tone into the `ses-ui` presentation tone.
///
/// Free function (not `From`) because the orphan rule rejects
/// `impl From<DeptTone> for ses_ui::ProgressTone`.
#[cfg(feature = "project")]
pub fn progress_tone(t: DeptProgressTone) -> ProgressTone {
    match t {
        DeptProgressTone::Neutral => ProgressTone::Neutral,
        DeptProgressTone::Good => ProgressTone::Good,
        DeptProgressTone::Warn => ProgressTone::Warn,
        DeptProgressTone::Over => ProgressTone::Over,
    }
}
