use crate::components::{PostList, ProjectList};
use crate::data::{get_all_posts, get_projects};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let posts =
        use_server_future(move || async move { get_all_posts().await.unwrap_or_default() })?;

    let projects = get_projects();

    rsx! {
        div {
            class: "page-content",

            div {
                class: "hero-section",
                h1 { "Hello." }
                p {
                    class: "bio",
                    "I'm a systems programmer interested in high-performance computing, numerical methods, and programming language design. I build things with Rust and C++, and occasionally write about the process."
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
