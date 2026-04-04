use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            class: "theme-toggle",
            onclick: |_| {
                document::eval(r#"
                    const html = document.documentElement;
                    const current = html.getAttribute('data-theme');
                    if (current === 'light') {
                        html.setAttribute('data-theme', 'dark');
                        localStorage.setItem('theme', 'dark');
                    } else {
                        html.setAttribute('data-theme', 'light');
                        localStorage.setItem('theme', 'light');
                    }
                "#);
            },
            "◐"
        }
    }
}
