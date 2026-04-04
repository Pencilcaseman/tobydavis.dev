use crate::data::TocEntry;
use dioxus::prelude::*;

#[component]
pub fn TableOfContents(entries: Vec<TocEntry>) -> Element {
    let min_level = entries.iter().map(|e| e.level).min().unwrap_or(1);

    rsx! {
        nav {
            class: "toc",
            div { class: "toc-title", "On this page" }
            for entry in entries.iter() {
                {
                    let depth = (entry.level - min_level) as usize;
                    let class = if depth > 0 { "toc-item toc-child" } else { "toc-item" };
                    let indent = format!("{}px", 12 + depth * 12);

                    rsx! {
                        a {
                            class: "{class}",
                            style: "padding-left: {indent}",
                            "data-level": "{entry.level}",
                            href: "#{entry.id}",
                            "{entry.title}"
                        }
                    }
                }
            }
        }
    }
}
