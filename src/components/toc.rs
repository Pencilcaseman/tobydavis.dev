use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TocEntry {
    pub id: String,
    pub title: String,
    pub level: u8,
}

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

                    let parent_active = if is_child {
                        entries.iter()
                            .rev()
                            .filter(|e| e.level == 2)
                            .take_while(|e| e.id != entry.id)
                            .any(|e| e.id == active_id)
                        || is_active
                    } else {
                        true
                    };

                    let class = format!(
                        "toc-item{}{}{}",
                        if is_child { " toc-h3" } else { "" },
                        if is_active { " active" } else { "" },
                        if is_child && !parent_active { " collapsed" } else { "" },
                    );

                    rsx! {
                        a {
                            class: "{class}",
                            href: "#{entry.id}",
                            "{entry.title}"
                        }
                    }
                }
            }
        }
    }
}
