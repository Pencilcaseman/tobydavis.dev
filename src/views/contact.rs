use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "Contact" }
            div {
                class: "contact-links",
                div {
                    class: "contact-row",
                    span { class: "contact-label", "GitHub" }
                    a { href: "https://github.com/Pencilcaseman", "Pencilcaseman" }
                }
                div {
                    class: "contact-row",
                    span { class: "contact-label", "Email" }
                    a { href: "mailto:pencilcaseman@gmail.com", "pencilcaseman@gmail.com" }
                }
            }
        }
    }
}
