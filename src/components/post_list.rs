use dioxus::prelude::*;

use crate::{data::PostMeta, misc::route::Route};

#[component]
pub fn PostList(posts: Vec<PostMeta>) -> Element {
    rsx! {
        div {
            class: "post-list",
            for post in posts.iter() {
                Link {
                    to: Route::BlogPost { id: post.id.clone() },
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
