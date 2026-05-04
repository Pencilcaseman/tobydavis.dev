use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "page-content",
            h1 { "About" }
            p {
                "I'm Toby Davis, a systems programmer interested in high-performance computing,
                numerical methods, and programming language design."
            }
            p {
                "I build things primarily with Rust and C++. My main project is "
                a { href: "https://github.com/LibRapid/librapid", "LibRapid" }
                ", a high-performance math library with SIMD acceleration."
            }
            p {
                "This site is built with Rust and Dioxus."
            }
        }
    }
}
