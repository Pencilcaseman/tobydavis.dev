use chrono::Datelike;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let year = chrono::Utc::now().year();

    rsx! {
        footer {
            class: "site-footer",
            div {
                class: "footer-content",
                span { "© 2023–{year} Toby Davis" }
                nav {
                    class: "footer-links",
                    a { href: "https://github.com/Pencilcaseman", "GitHub" }
                    a { href: "mailto:pencilcaseman@gmail.com", "Email" }
                }
            }
        }
    }
}
