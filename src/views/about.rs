use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "About" }
            p { "Coming soon." }
        }
    }
}
