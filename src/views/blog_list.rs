use crate::components::PostList;
use crate::data::get_all_posts;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    let posts = use_server_future(move || async move {
        get_all_posts().await.unwrap_or_default()
    })?;

    rsx! {
        div {
            class: "page-content",
            h1 { "Blog" }
            match &*posts.read() {
                Some(posts) => rsx! { PostList { posts: posts.clone() } },
                None => rsx! { p { class: "loading", "Loading..." } },
            }
        }
    }
}
