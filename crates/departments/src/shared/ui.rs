//! Cross-department UI helpers.

#[cfg(feature = "project")]
use crate::project::progress::ProgressTone as DeptProgressTone;
#[cfg(feature = "project")]
use ses_ui::ProgressTone;
use ses_shell::{ShellState, WorkspaceBinding, WorkspaceDef};

/// Open `ws`, or focus the existing bound tab if this template is already
/// instantiated for the same binding value. One implementation shared by
/// every page that opens a project workspace — do not duplicate this match.
pub fn apply_open_workspace(shell: &mut ShellState, ws: WorkspaceDef) {
    if let Some(existing) = ws.template_of.as_ref().and_then(|m| {
        ws.binding
            .get(WorkspaceBinding::PROJECT_ID)
            .and_then(|v| shell.find_bound(m, WorkspaceBinding::PROJECT_ID, v))
    }) {
        shell.set_active(existing);
        shell.status_message = "Switched to project workspace".into();
        return;
    }
    shell.add_workspace(ws);
    shell.status_message = "Opened project workspace".into();
}

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
