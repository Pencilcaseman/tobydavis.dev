use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "Blog" }
            p { "Coming soon." }
        }
    }
}
