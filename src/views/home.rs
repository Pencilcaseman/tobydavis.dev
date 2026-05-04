use dioxus::prelude::*;

use crate::{
    components::{PostList, ProjectList},
    data::{get_all_posts, get_projects},
};

const BIO_TEXT: &str = r"
    What should I put here? Probably something informative and useful. Does Rust
    have string concatenation?
";

#[component]
pub fn Home() -> Element {
    let posts = use_server_future(move || async move {
        get_all_posts().await.unwrap_or_default()
    })?;

    let projects = get_projects();

    rsx! {
        div {
            class: "page-content",

            div {
                class: "hero-section",
                h1 { "What Goes Here?" }
                p {
                    class: "bio",
                    { BIO_TEXT }
                }
            }

            div {
                class: "section",
                h2 { class: "section-head", "Recent Posts" }
                match &*posts.read() {
                    Some(posts) => rsx! { PostList { posts: posts.clone() } },
                    None => rsx! { p { class: "loading", "Loading..." } },
                }
            }

            div {
                class: "section",
                h2 { class: "section-head", "Projects" }
                ProjectList { projects }
            }
        }
    }
}
