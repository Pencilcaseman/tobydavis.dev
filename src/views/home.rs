use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "Hello." }
            p { "Welcome to my site. More content coming soon." }
        }
    }
}
