use dioxus::prelude::*;
use crate::HistoryEntry;

#[component]
pub fn RecipeHistory(entries: &'static [HistoryEntry]) -> Element {
    let mut show_history = use_signal(|| false);

    if entries.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "recipe-history",
            button {
                class: "history-toggle",
                onclick: move |_| show_history.toggle(),
                if show_history() {
                    "▼ Recipe History ({entries.len()} versions)"
                } else {
                    "▶ Recipe History ({entries.len()} versions)"
                }
            }
            if show_history() {
                div {
                    class: "history-entries",
                    for entry in entries {
                        div {
                            class: "history-entry",
                            h3 { "{entry.date} - {entry.title}" }
                            div {
                                class: "prose",
                                dangerous_inner_html: "{entry.notes_html}"
                            }
                        }
                    }
                }
            }
        }
    }
}
