use crate::components::{TableOfContents, TocEntry};
use crate::data::get_post;
use dioxus::prelude::*;

#[component]
pub fn BlogPost(slug: String) -> Element {
    let post = use_server_future(move || {
        let slug = slug.clone();
        async move { get_post(slug).await }
    })?;

    match &*post.read() {
        Some(Ok(data)) => {
            let toc_entries: Vec<TocEntry> = data
                .toc
                .iter()
                .map(|t| TocEntry {
                    id: t.id.clone(),
                    title: t.title.clone(),
                    level: t.level,
                })
                .collect();

            let first_id = toc_entries
                .first()
                .map(|e| e.id.clone())
                .unwrap_or_default();

            rsx! {
                div {
                    class: "blog-layout",

                    TableOfContents {
                        entries: toc_entries,
                        active_id: first_id,
                    }

                    article {
                        class: "blog-content",

                        h1 { "{data.meta.title}" }
                        div {
                            class: "post-meta",
                            "Toby Davis · {data.meta.date} · {data.meta.reading_time_minutes} min read"
                        }

                        div {
                            dangerous_inner_html: "{data.html}",
                        }
                    }

                    div { class: "blog-spacer" }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                div {
                    class: "page-content",
                    h1 { "Post not found" }
                    p { "Could not load post: {e}" }
                }
            }
        }
        None => {
            rsx! {
                div {
                    class: "page-content",
                    p { class: "loading", "Loading..." }
                }
            }
        }
    }
}
