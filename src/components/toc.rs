use dioxus::prelude::*;

use crate::{components::icons::ChevronRight, data::TocEntry};

#[component]
pub fn TableOfContents(
    entries: Vec<TocEntry>,
    #[props(default = String::new())] modifier: String,
    #[props(default = String::new())] id: String,
) -> Element {
    let min_level = entries.iter().map(|e| e.level).min().unwrap_or(1);

    let has_children: Vec<bool> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| {
            entries.get(i + 1).is_some_and(|next| next.level > e.level)
        })
        .collect();

    let nav_class = if modifier.is_empty() {
        "toc".to_string()
    } else {
        format!("toc {modifier}")
    };

    rsx! {
        nav {
            class: "{nav_class}",
            id: "{id}",
            div { class: "toc-title", "On this page" }
            for (i, entry) in entries.iter().enumerate() {
                {
                    let depth = (entry.level - min_level) as usize;
                    let indent = format!("{}px", 12 + depth * 12);

                    let row_class = if depth > 0 {
                        "toc-row child-row"
                    } else {
                        "toc-row"
                    };

                    let item_class = if depth > 0 {
                        "toc-item toc-child"
                    } else {
                        "toc-item"
                    };

                    rsx! {
                        div {
                            class: "{row_class}",
                            a {
                                class: "{item_class}",
                                style: "padding-left: {indent}",
                                "data-level": "{entry.level}",
                                href: "#{entry.id}",
                                "{entry.title}"
                            }
                            if has_children[i] {
                                button {
                                    class: "toc-toggle",
                                    "aria-label": "Toggle section",
                                    ChevronRight { size: "14px".to_string() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
