use crate::components::TableOfContents;
use crate::data::get_post;
use dioxus::prelude::*;

const SCROLL_OBSERVER_JS: &str = r##"
    function init() {
        const headings = document.querySelectorAll('.blog-content [id]');
        const tocItems = Array.from(document.querySelectorAll('.toc-item'));
        if (headings.length === 0 || tocItems.length === 0) return;

        // Get the children (toc-child items) that follow a parent item
        function getChildren(parent) {
            const items = [];
            let el = parent.nextElementSibling;
            while (el && el.classList.contains('toc-child')) {
                items.push(el);
                el = el.nextElementSibling;
            }
            return items;
        }

        // Walk backwards to find the nearest non-child ancestor
        function getParent(child) {
            let el = child.previousElementSibling;
            while (el && el.classList.contains('toc-child')) el = el.previousElementSibling;
            return el;
        }

        function activate(id) {
            tocItems.forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.toc-child.visible').forEach(el => el.classList.remove('visible'));

            const link = document.querySelector(`.toc-item[href="#${id}"]`);
            if (!link) return;
            link.classList.add('active');

            if (link.classList.contains('toc-child')) {
                // Activate parent and show all siblings
                const parent = getParent(link);
                if (parent) {
                    parent.classList.add('active');
                    getChildren(parent).forEach(el => el.classList.add('visible'));
                }
            } else {
                // Show children of this parent
                getChildren(link).forEach(el => el.classList.add('visible'));
            }
        }

        const observer = new IntersectionObserver(entries => {
            for (const entry of entries) {
                if (entry.isIntersecting) activate(entry.target.id);
            }
        }, { rootMargin: '0px 0px -70% 0px' });

        headings.forEach(h => observer.observe(h));
    }

    if (document.readyState === 'complete') {
        setTimeout(init, 50);
    } else {
        window.addEventListener('load', () => setTimeout(init, 50));
    }
"##;

#[component]
pub fn BlogPost(id: String) -> Element {
    use_effect(|| { document::eval(SCROLL_OBSERVER_JS); });

    let post = use_server_future(move || {
        let id = id.clone();
        async move { get_post(id).await }
    })?;

    match &*post.read() {
        Some(Ok(data)) => rsx! {
            div {
                class: "blog-layout",
                TableOfContents { entries: data.toc.clone() }
                article {
                    class: "blog-content",
                    h1 { "{data.meta.title}" }
                    div { class: "post-meta", "Toby Davis · {data.meta.date} · {data.meta.reading_time_minutes} min read" }
                    div { dangerous_inner_html: "{data.html}" }
                }
                div { class: "blog-spacer" }
            }
        },
        Some(Err(e)) => rsx! {
            div { class: "page-content",
                h1 { "Post not found" }
                p { "Could not load post: {e}" }
            }
        },
        None => rsx! {
            div { class: "page-content", p { class: "loading", "Loading..." } }
        },
    }
}
