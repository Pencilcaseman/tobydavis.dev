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
        Home {},
        #[route("/blog")]
        BlogList {},
        #[route("/blog/:slug")]
        BlogPost { slug: String },
        #[route("/projects")]
        Projects {},
        #[route("/about")]
        About {},
        #[route("/contact")]
        Contact {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const HOME_CSS: Asset = asset!("/assets/styling/home.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        document::Link { rel: "stylesheet", href: HOME_CSS }

        Router::<Route> {}
    }
}
