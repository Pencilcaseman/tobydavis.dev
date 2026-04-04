use crate::components::TableOfContents;
use crate::data::get_post;
use dioxus::prelude::*;

const SCROLL_OBSERVER_JS: &str = r##"
    const headings = document.querySelectorAll('.blog-content h2[id], .blog-content h3[id]');
    if (headings.length === 0) return;

    function getH3Group(h2Link) {
        const items = [];
        let el = h2Link.nextElementSibling;
        while (el && el.classList.contains('toc-h3')) {
            items.push(el);
            el = el.nextElementSibling;
        }
        return items;
    }

    function getParentH2(h3Link) {
        let el = h3Link.previousElementSibling;
        while (el && el.classList.contains('toc-h3')) el = el.previousElementSibling;
        return el;
    }

    function activate(id) {
        document.querySelectorAll('.toc-item.active').forEach(el => el.classList.remove('active'));
        document.querySelectorAll('.toc-item.toc-h3.visible').forEach(el => el.classList.remove('visible'));

        const link = document.querySelector(`.toc-item[href="#${id}"]`);
        if (!link) return;
        link.classList.add('active');

        if (link.classList.contains('toc-h3')) {
            const parent = getParentH2(link);
            if (parent) {
                parent.classList.add('active');
                getH3Group(parent).forEach(el => el.classList.add('visible'));
            }
        } else {
            getH3Group(link).forEach(el => el.classList.add('visible'));
        }
    }

    const observer = new IntersectionObserver(entries => {
        for (const entry of entries) {
            if (entry.isIntersecting) activate(entry.target.id);
        }
    }, { rootMargin: '0px 0px -70% 0px' });

    headings.forEach(h => observer.observe(h));
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
