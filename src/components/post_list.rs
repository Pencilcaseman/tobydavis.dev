use crate::data::PostMeta;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PostList(posts: Vec<PostMeta>) -> Element {
    rsx! {
        div {
            class: "post-list",
            for post in posts.iter() {
                Link {
                    to: Route::BlogPost { slug: post.slug.clone() },
                    class: "post-row",
                    div {
                        div { class: "post-title", "{post.title}" }
                        div { class: "post-desc", "{post.description}" }
                    }
                    span { class: "post-date", "{post.date}" }
                }
            }
        }
    }
}
