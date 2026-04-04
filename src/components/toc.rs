use crate::data::TocEntry;
use dioxus::prelude::*;

#[component]
pub fn TableOfContents(entries: Vec<TocEntry>) -> Element {
    rsx! {
        nav {
            class: "toc",
            div { class: "toc-title", "On this page" }
            for entry in entries.iter() {
                a {
                    class: if entry.level == 3 { "toc-item toc-h3" } else { "toc-item" },
                    href: "#{entry.id}",
                    "{entry.title}"
                }
            }
        }
    }
}
