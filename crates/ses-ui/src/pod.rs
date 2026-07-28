//! Pod components — functional editors.

pub mod app_ribbon;
pub mod calculation;
pub mod menu_bar;
pub mod outliner;
pub mod properties;
pub mod status_bar;
pub mod top_bar;
pub mod view;

use dioxus::prelude::*;
use ses_shell::PodKind;

use calculation::CalculationPod;
use menu_bar::MenuBarPod;
use outliner::OutlinerPod;
use properties::PropertiesPod;
use view::ViewPod;

#[component]
pub fn PodHost(kind: PodKind, channel: String) -> Element {
    match kind {
        PodKind::View => rsx! { ViewPod {} },
        PodKind::Outliner => rsx! { OutlinerPod {} },
        PodKind::Properties => rsx! { PropertiesPod {} },
        PodKind::Calculation => rsx! { CalculationPod { channel } },
        PodKind::MenuBar => rsx! { MenuBarPod {} },
        PodKind::TopBar | PodKind::StatusBar => rsx! {
            div { class: "ses-pod",
                p { class: "ses-muted", "Global chrome pod — rendered by Screen." }
            }
        },
    }
}

pub use status_bar::StatusBarPod;
pub use top_bar::TopBarPod;
