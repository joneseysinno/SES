//! Renders a PageDescriptor by dispatching to its module UI.

use crate::context::{use_modules_ui, use_shell, use_user};
use crate::module_ui::PageCtx;
use dioxus::prelude::*;
use ses_shell::PageDescriptor;

#[component]
pub fn PageView(page: PageDescriptor, leaf_id: ses_shell::LeafId) -> Element {
    let shell = use_shell();
    let modules = use_modules_ui();
    let user = use_user();

    let (workspace_id, binding) = {
        let s = shell.read();
        let ws = s.active();
        (
            s.active_workspace,
            ws.map(|w| w.binding.clone()).unwrap_or_default(),
        )
    };

    let ctx = PageCtx {
        workspace_id,
        leaf_id,
        binding,
        user: user.read().clone(),
    };

    modules.read().render_page(&page.module_id, &page.page_id, &ctx)
}
