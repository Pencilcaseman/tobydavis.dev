use crate::data::TocEntry;
use dioxus::prelude::*;

#[component]
pub fn TableOfContents(entries: Vec<TocEntry>, active_id: String) -> Element {
    rsx! {
        nav {
            class: "toc",
            div { class: "toc-title", "On this page" }
            for entry in entries.iter() {
                {
                    let is_active = entry.id == active_id;
                    let is_child = entry.level == 3;
                    let parent_active = !is_child || is_active || entries.iter()
                        .rev()
                        .filter(|e| e.level == 2)
                        .take_while(|e| e.id != entry.id)
                        .any(|e| e.id == active_id);

                    let class = format!(
                        "toc-item{}{}{}",
                        if is_child { " toc-h3" } else { "" },
                        if is_active { " active" } else { "" },
                        if is_child && !parent_active { " collapsed" } else { "" },
                    );

                    rsx! {
                        a { class: "{class}", href: "#{entry.id}", "{entry.title}" }
                    }
                }
            }
        }
    }
}
