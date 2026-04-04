use crate::components::TableOfContents;
use crate::data::get_post;
use dioxus::prelude::*;

#[component]
pub fn BlogPost(id: String) -> Element {
    let post = use_server_future(move || {
        let id = id.clone();
        async move { get_post(id).await }
    })?;

    match &*post.read() {
        Some(Ok(data)) => {
            let first_id = data.toc.first().map(|e| e.id.clone()).unwrap_or_default();

            rsx! {
                div {
                    class: "blog-layout",
                    TableOfContents { entries: data.toc.clone(), active_id: first_id }
                    article {
                        class: "blog-content",
                        h1 { "{data.meta.title}" }
                        div { class: "post-meta", "Toby Davis · {data.meta.date} · {data.meta.reading_time_minutes} min read" }
                        div { dangerous_inner_html: "{data.html}" }
                    }
                    div { class: "blog-spacer" }
                }
            }
        }
        Some(Err(e)) => rsx! {
            div { class: "page-content",
                h1 { "Post not found" }
                p { "Could not load post: {e}" }
            }
        },
        None => rsx! {
            div { class: "page-content", p { class: "loading", "Loading..." } }
        },
    }
}
