use crate::project_management::bridge::{NewProjectParams, ProjectFilter, ProjectMgmtCommand, ProjectMgmtQuery};
use crate::project_management::numbering::next_number;
use crate::shared::{now_utc, parse_date_utc, year_utc};
use crate::store::{use_dept_store, MgmtQueryResult};
use dioxus::prelude::*;
use ses_ui::{use_shell, DateInput, FieldMeta, Modal, NumericalInput, TextInput};

/// Modal form for `ProjectMgmtCommand::CreateProject`. The number is
/// prefilled from [`next_number`] but stays editable; a duplicate is
/// reported inline, not on the status bar.
#[component]
pub fn NewProjectDialog(on_close: EventHandler<()>) -> Element {
    let mut store = use_dept_store();
    let mut shell = use_shell();

    let existing_numbers: Vec<String> = match store
        .read()
        .query_mgmt(ProjectMgmtQuery::ListAll {
            filter: ProjectFilter::default(),
        }) {
        Ok(MgmtQueryResult::Projects(p)) => p.into_iter().map(|p| p.number).collect(),
        _ => Vec::new(),
    };
    let year = year_utc(now_utc());

    let mut number = use_signal(|| next_number(year, existing_numbers.iter().map(String::as_str)));
    let mut name = use_signal(String::new);
    let mut client = use_signal(String::new);
    let mut manager = use_signal(String::new);
    let mut target_finish = use_signal(String::new);
    let mut contract_value = use_signal(String::new);
    let mut number_error = use_signal(|| None::<String>);
    let mut name_error = use_signal(|| None::<String>);

    let submit = move |_| {
        let num = number.read().trim().to_string();
        let nm = name.read().trim().to_string();

        if store.read().projects.values().any(|p| p.number == num) {
            number_error.set(Some("A project already uses this number".into()));
            return;
        }
        if nm.is_empty() {
            name_error.set(Some("Name is required".into()));
            return;
        }

        let client_val = client.read().trim().to_string();
        let manager_val = manager.read().trim().to_string();
        let target_finish_utc = parse_date_utc(target_finish.read().trim());
        let contract_value_cents = contract_value
            .read()
            .trim()
            .parse::<f64>()
            .ok()
            .map(|v| (v * 100.0).round() as i64);

        let result = store
            .write()
            .execute_mgmt(ProjectMgmtCommand::CreateProject(NewProjectParams {
                name: nm,
                number: num,
                client: (!client_val.is_empty()).then_some(client_val),
                manager: (!manager_val.is_empty()).then_some(manager_val),
                target_finish_utc,
                contract_value_cents,
            }));
        match result {
            Ok(_) => {
                shell.write().status_message = "Created project".into();
                on_close.call(());
            }
            Err(e) => shell.write().status_message = format!("Create project failed: {e}"),
        }
    };

    rsx! {
        Modal {
            title: "New Project".to_string(),
            open: true,
            on_close: move |_| on_close.call(()),
            TextInput {
                id: "new-project-number".to_string(),
                label: "Number".to_string(),
                value: number(),
                error: number_error(),
                oninput: move |v: String| {
                    number.set(v);
                    number_error.set(None);
                },
            }
            TextInput {
                id: "new-project-name".to_string(),
                label: "Name".to_string(),
                value: name(),
                error: name_error(),
                oninput: move |v: String| {
                    name.set(v);
                    name_error.set(None);
                },
            }
            TextInput {
                id: "new-project-client".to_string(),
                label: "Client".to_string(),
                value: client(),
                oninput: move |v: String| client.set(v),
            }
            TextInput {
                id: "new-project-manager".to_string(),
                label: "Manager".to_string(),
                value: manager(),
                oninput: move |v: String| manager.set(v),
            }
            DateInput {
                meta: FieldMeta::new("new-project-target", "Target finish"),
                value: target_finish(),
                oninput: move |v: String| target_finish.set(v),
            }
            NumericalInput {
                id: "new-project-value".to_string(),
                label: "Contract value".to_string(),
                value: contract_value(),
                units: Some("USD".to_string()),
                oninput: move |v: String| contract_value.set(v),
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
