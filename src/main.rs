use dioxus::prelude::*;

use views::{About, BlogList, BlogPost, Contact, Home, Navbar, Projects};

mod components;
mod data;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home,
        #[route("/blog")]
        BlogList,
        #[route("/blog/:id")]
        BlogPost { id: String },
        #[route("/projects")]
        Projects,
        #[route("/about")]
        About,
        #[route("/contact")]
        Contact,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const HOME_CSS: Asset = asset!("/assets/styling/home.css");
const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(|| {
        document::eval(
            r#"
            const saved = localStorage.getItem('theme');
            if (saved) {
                document.documentElement.setAttribute('data-theme', saved);
            }
        "#,
        );
    });

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        document::Link { rel: "stylesheet", href: HOME_CSS }
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        Router::<Route> {}
    }
}
