use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "Projects" }
            p { "Coming soon." }
        }
    }
}
