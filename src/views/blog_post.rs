use crate::components::TableOfContents;
use crate::data::get_post;
use dioxus::prelude::*;

const SCROLL_OBSERVER_JS: &str = r##"
    function init() {
        const headings = document.querySelectorAll('.blog-content [id]');
        const tocItems = Array.from(document.querySelectorAll('.toc-item'));
        if (headings.length === 0 || tocItems.length === 0) return;

        // Track which parents are manually pinned open/closed
        const pinned = new Map();

        function getRow(item) {
            return item.closest('.toc-row');
        }

        function getChildren(parentItem) {
            const items = [];
            let row = getRow(parentItem)?.nextElementSibling;
            while (row) {
                const child = row.querySelector('.toc-child');
                if (!child) break;
                items.push(child);
                row = row.nextElementSibling;
            }
            return items;
        }

        function getParentItem(childItem) {
            let row = getRow(childItem)?.previousElementSibling;
            while (row) {
                const item = row.querySelector('.toc-item:not(.toc-child)');
                if (item) return item;
                row = row.previousElementSibling;
            }
            return null;
        }

        function showChildren(parentItem) {
            getChildren(parentItem).forEach(el => el.classList.add('visible'));
            const btn = getRow(parentItem)?.querySelector('.toc-toggle');
            if (btn) btn.classList.add('expanded');
        }

        function hideChildren(parentItem) {
            getChildren(parentItem).forEach(el => el.classList.remove('visible'));
            const btn = getRow(parentItem)?.querySelector('.toc-toggle');
            if (btn) btn.classList.remove('expanded');
        }

        function activate(id) {
            tocItems.forEach(el => el.classList.remove('active'));

            // Collapse all non-pinned sections
            tocItems.filter(el => !el.classList.contains('toc-child')).forEach(parent => {
                const href = parent.getAttribute('href')?.slice(1);
                if (!pinned.get(href)) hideChildren(parent);
            });

            const link = document.querySelector(`.toc-item[href="#${id}"]`);
            if (!link) return;
            link.classList.add('active');

            if (link.classList.contains('toc-child')) {
                const parent = getParentItem(link);
                if (parent) {
                    parent.classList.add('active');
                    showChildren(parent);
                }
            } else {
                showChildren(link);
            }

            // Also show any pinned sections
            pinned.forEach((open, href) => {
                if (open) {
                    const item = document.querySelector(`.toc-item[href="#${href}"]`);
                    if (item) showChildren(item);
                }
            });
        }

        // Toggle buttons
        document.querySelectorAll('.toc-toggle').forEach(btn => {
            btn.addEventListener('click', e => {
                e.preventDefault();
                e.stopPropagation();
                const row = btn.closest('.toc-row');
                const parentItem = row?.querySelector('.toc-item');
                if (!parentItem) return;
                const href = parentItem.getAttribute('href')?.slice(1);

                const isExpanded = btn.classList.contains('expanded');
                if (isExpanded) {
                    hideChildren(parentItem);
                    pinned.set(href, false);
                } else {
                    showChildren(parentItem);
                    pinned.set(href, true);
                }
            });
        });

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
