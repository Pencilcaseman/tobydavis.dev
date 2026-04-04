use crate::components::{Footer, ThemeToggle};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "site-nav",
            Link {
                to: Route::Home {},
                class: "site-name",
                "Toby Davis"
            }
            span { class: "nav-sep", "·" }
            div {
                class: "nav-links",
                Link { to: Route::Home {}, "Home" }
                Link { to: Route::BlogList {}, "Blog" }
                Link { to: Route::Projects {}, "Projects" }
                Link { to: Route::About {}, "About" }
                Link { to: Route::Contact {}, "Contact" }
            }
            ThemeToggle {}
        }

        Outlet::<Route> {}

        Footer {}
    }
}
