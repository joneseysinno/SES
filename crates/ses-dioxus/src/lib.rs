pub mod app_state;
pub mod views;

use app_state::AppState;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(AppState::new);

    rsx! {
        views::home::Home {}
    }
}
