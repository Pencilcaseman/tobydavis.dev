use crate::data::ProjectMeta;
use dioxus::prelude::*;

#[component]
pub fn ProjectList(projects: Vec<ProjectMeta>) -> Element {
    rsx! {
        div {
            class: "project-list",
            for project in projects.iter() {
                div {
                    class: "proj-row",
                    a {
                        class: "proj-name",
                        href: "{project.url}",
                        "{project.name}"
                    }
                    div { class: "proj-desc", "{project.description}" }
                    div { class: "proj-tech", "{project.tech}" }
                }
            }
        }
    }
}
