use dioxus::prelude::*;

use crate::{components::ProjectList, data::get_projects};

#[component]
pub fn Projects() -> Element {
    let projects = get_projects();

    rsx! {
        div {
            class: "page-content",
            h1 { "Projects" }
            ProjectList { projects }
        }
    }
}
