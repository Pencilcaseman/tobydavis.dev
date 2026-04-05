use crate::components::icons::ChevronRight;
use crate::data::TocEntry;
use dioxus::prelude::*;

#[component]
pub fn TableOfContents(entries: Vec<TocEntry>) -> Element {
    let min_level = entries.iter().map(|e| e.level).min().unwrap_or(1);

    let has_children: Vec<bool> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| entries.get(i + 1).is_some_and(|next| next.level > e.level))
        .collect();

    rsx! {
        nav {
            class: "toc",
            div { class: "toc-title", "On this page" }
            for (i, entry) in entries.iter().enumerate() {
                {
                    let depth = (entry.level - min_level) as usize;
                    let row_class = if depth > 0 { "toc-row child-row" } else { "toc-row" };
                    let item_class = if depth > 0 { "toc-item toc-child" } else { "toc-item" };
                    let indent = format!("{}px", 12 + depth * 12);

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
