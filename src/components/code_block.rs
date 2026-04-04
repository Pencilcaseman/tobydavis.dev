use dioxus::prelude::*;

#[component]
pub fn CodeBlock(code: String, language: Option<String>) -> Element {
    let lang_class = language
        .as_deref()
        .map(|l| format!("language-{l}"))
        .unwrap_or_default();

    rsx! {
        div {
            class: "code-block-wrapper",
            if let Some(lang) = &language {
                span { class: "code-lang-label", "{lang}" }
            }
            pre {
                code {
                    class: "{lang_class}",
                    "{code}"
                }
            }
        }
    }
}
