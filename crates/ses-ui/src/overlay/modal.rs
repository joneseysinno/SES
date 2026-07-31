//! Modal overlay shell — backdrop, Esc-to-close, centered card.

use dioxus::prelude::*;

#[component]
pub fn Modal(
    title: String,
    open: bool,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "ses-modal-root",
            role: "presentation",
            onkeydown: move |evt| {
                if evt.key() == Key::Escape {
                    on_close.call(());
                }
            },
            div {
                class: "ses-modal-backdrop",
                onclick: move |_| on_close.call(()),
            }
            div {
                class: "ses-modal",
                role: "dialog",
                aria_modal: "true",
                aria_label: "{title}",
                tabindex: "0",
                onmounted: move |e| {
                    let data = e.data();
                    spawn(async move {
                        let _ = data.set_focus(true).await;
                    });
                },
                onclick: move |evt| evt.stop_propagation(),
                div { class: "ses-modal-header",
                    h2 { class: "ses-modal-title", "{title}" }
                    button {
                        class: "ses-ghost ses-modal-close",
                        r#type: "button",
                        title: "Close",
                        onclick: move |_| on_close.call(()),
                        "×"
                    }
                }
                div { class: "ses-modal-body",
                    {children}
                }
            }
        }
    }
}
