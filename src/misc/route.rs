use dioxus::prelude::*;

use crate::views::{
    About, BlogList, BlogPost, Contact, Home, Navbar, Projects,
};

#[derive(Routable, Debug, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog")]
        BlogList {},
        #[route("/blog/:id")]
        BlogPost { id: String },
        #[route("/projects")]
        Projects {},
        #[route("/about")]
        About {},
        #[route("/contact")]
        Contact {},
}
