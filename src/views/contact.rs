use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "Contact" }
            p { "Coming soon." }
        }
    }
}
