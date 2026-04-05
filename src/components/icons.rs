use dioxus::prelude::*;

#[component]
pub fn ChevronRight(#[props(default = "1em".to_string())] size: String) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "m8.25 4.5 7.5 7.5-7.5 7.5",
            }
        }
    }
}
