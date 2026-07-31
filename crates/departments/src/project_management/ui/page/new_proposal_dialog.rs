use crate::project_management::bridge::{
    NewProjectParams, NewProposalParams, ProjectMgmtCommand,
};
use crate::project_management::numbering::next_number;
use crate::project_management::payloads::ProjectPhase;
use crate::shared::{now_utc, year_utc};
use crate::store::use_dept_store;
use dioxus::prelude::*;
use ses_ui::{use_shell, Modal, NumericalInput, TextInput};

/// Modal form for a proposal: creates the project record in `Proposal`
/// phase, then attaches the proposal payload — the same two-step sequence
/// the top-bar scaffolding used, now with real field values.
#[component]
pub fn NewProposalDialog(on_close: EventHandler<()>) -> Element {
    let mut store = use_dept_store();
    let mut shell = use_shell();

    let mut client = use_signal(String::new);
    let mut name = use_signal(String::new);
    let mut scope = use_signal(String::new);
    let mut fee = use_signal(String::new);
    let mut name_error = use_signal(|| None::<String>);

    let submit = move |_| {
        let nm = name.read().trim().to_string();
        if nm.is_empty() {
            name_error.set(Some("Name is required".into()));
            return;
        }
        let client_val = client.read().trim().to_string();
        let fee_cents = fee
            .read()
            .trim()
            .parse::<f64>()
            .ok()
            .map(|v| (v * 100.0).round() as i64);

        let existing_numbers: Vec<String> =
            store.read().projects.values().map(|p| p.number.clone()).collect();
        let year = year_utc(now_utc());
        let number = next_number(year, existing_numbers.iter().map(String::as_str));

        let create = store
            .write()
            .execute_mgmt(ProjectMgmtCommand::CreateProject(NewProjectParams {
                name: nm,
                number,
                client: (!client_val.is_empty()).then_some(client_val),
                ..Default::default()
            }));
        let pid = match create {
            Ok(_) => store
                .read()
                .projects
                .keys()
                .copied()
                .max_by_key(|id| id.0)
                .expect("just created"),
            Err(e) => {
                shell.write().status_message = format!("Create proposal failed: {e}");
                return;
            }
        };
        let _ = store.write().execute_mgmt(ProjectMgmtCommand::SetPhase {
            id: pid,
            phase: ProjectPhase::Proposal,
        });
        let result = store
            .write()
            .execute_mgmt(ProjectMgmtCommand::CreateProposal(NewProposalParams {
                project_id: pid,
                scope: scope.read().clone(),
                fee_cents,
            }));
        match result {
            Ok(_) => {
                shell.write().status_message = "Created proposal".into();
                on_close.call(());
            }
            Err(e) => shell.write().status_message = format!("Create proposal failed: {e}"),
        }
    };

    rsx! {
        Modal {
            title: "New Proposal".to_string(),
            open: true,
            on_close: move |_| on_close.call(()),
            TextInput {
                id: "new-proposal-client".to_string(),
                label: "Client".to_string(),
                value: client(),
                oninput: move |v: String| client.set(v),
            }
            TextInput {
                id: "new-proposal-name".to_string(),
                label: "Name".to_string(),
                value: name(),
                error: name_error(),
                oninput: move |v: String| {
                    name.set(v);
                    name_error.set(None);
                },
            }
            div { class: "ses-io-field",
                div { class: "ses-io-field-row",
                    label { class: "ses-io-field-label", r#for: "new-proposal-scope", "Scope" }
                }
                textarea {
                    id: "new-proposal-scope",
                    class: "ses-io-field-control",
                    rows: "4",
                    value: "{scope()}",
                    oninput: move |e: Event<FormData>| scope.set(e.value()),
                }
            }
            NumericalInput {
                id: "new-proposal-fee".to_string(),
                label: "Fee".to_string(),
                value: fee(),
                units: Some("USD".to_string()),
                oninput: move |v: String| fee.set(v),
            }
            div { class: "ses-modal-actions",
                button {
                    class: "ses-ghost",
                    r#type: "button",
                    onclick: move |_| on_close.call(()),
                    "Cancel"
                }
                button {
                    r#type: "button",
                    onclick: submit,
                    "Create"
                }
            }
        }
    }
}
