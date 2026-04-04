use crate::components::ProjectList;
use crate::data::get_projects;
use dioxus::prelude::*;

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
