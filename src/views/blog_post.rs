use dioxus::prelude::*;

#[component]
pub fn BlogPost(slug: String) -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "Post: {slug}" }
            p { "Coming soon." }
        }
    }
}
