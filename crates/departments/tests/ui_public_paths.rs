//! Public-path smoke tests for department UI workspace factories.

use ses_shell::PageFilter;

#[test]
fn project_management_portfolio_is_reachable() {
    let ws = departments::project_management::ui::workspace::portfolio();
    assert_eq!(ws.seed_key.as_deref(), Some("project-mgmt/main"));
    assert_eq!(
        ws.page_filter,
        PageFilter::Department(ses_shell::ModuleId::new("project-mgmt"))
    );
}

#[test]
fn project_template_is_reachable() {
    let ws = departments::project::ui::workspace::template();
    assert_eq!(ws.seed_key.as_deref(), Some("project/template"));
    assert_eq!(
        ws.page_filter,
        PageFilter::Department(ses_shell::ModuleId::new("project"))
    );
}
