use dioxus::prelude::*;

use crate::{components::TableOfContents, data::get_post};

const SCROLL_OBSERVER_JS: &str = r##"
    function init() {
        const headings = document.querySelectorAll('.blog-content [id]');
        if (!headings.length) return;

        function setupToc(toc) {
            const rows = Array.from(toc.querySelectorAll('.toc-row'));
            if (!rows.length) return null;

            const pinned = new Map(); // href -> bool

            const itemOf = r => r.querySelector('.toc-item');
            const levelOf = r => parseInt(itemOf(r)?.dataset.level || '0');
            const hrefOf = r => itemOf(r)?.getAttribute('href')?.slice(1) || '';
            const isChild = r => r.classList.contains('child-row');

            function directChildren(row) {
                const lvl = levelOf(row), out = [];
                for (let i = rows.indexOf(row) + 1; i < rows.length; i++) {
                    const l = levelOf(rows[i]);
                    if (l <= lvl) break;
                    if (l === lvl + 1) out.push(rows[i]);
                }
                return out;
            }

            function allDescendants(row) {
                const lvl = levelOf(row), out = [];
                for (let i = rows.indexOf(row) + 1; i < rows.length; i++) {
                    if (levelOf(rows[i]) <= lvl) break;
                    out.push(rows[i]);
                }
                return out;
            }

            function parent(row) {
                const lvl = levelOf(row);
                for (let i = rows.indexOf(row) - 1; i >= 0; i--) {
                    if (levelOf(rows[i]) < lvl) return rows[i];
                }
                return null;
            }

            function ancestorChain(row) {
                const chain = [];
                let r = row;
                while (r) { chain.push(r); r = parent(r); }
                return chain;
            }

            const reveal = row => row.classList.add('visible');
            const conceal = row => row.classList.remove('visible');

            function syncToggles() {
                rows.forEach(row => {
                    const btn = row.querySelector('.toc-toggle');
                    if (!btn) return;
                    const hasVisibleChild = directChildren(row).some(c => c.classList.contains('visible'));
                    btn.classList.toggle('expanded', hasVisibleChild);
                });
            }

            function activate(id) {
                rows.forEach(r => itemOf(r)?.classList.remove('active'));

                rows.filter(isChild).forEach(r => {
                    if (!pinned.get(hrefOf(r))) conceal(r);
                });

                const link = toc.querySelector(`.toc-item[href="#${id}"]`);
                if (!link) { syncToggles(); return; }
                link.classList.add('active');
                const activeRow = link.closest('.toc-row');
                if (!activeRow) { syncToggles(); return; }

                const chain = ancestorChain(activeRow);
                for (const row of chain) {
                    reveal(row);
                    const p = parent(row);
                    if (p) directChildren(p).forEach(reveal);
                }
                directChildren(activeRow).forEach(reveal);

                pinned.forEach((open, href) => {
                    if (!open) return;
                    const el = toc.querySelector(`.toc-item[href="#${href}"]`)?.closest('.toc-row');
                    if (el) directChildren(el).forEach(reveal);
                });

                syncToggles();
            }

            toc.querySelectorAll('.toc-toggle').forEach(btn => {
                btn.addEventListener('click', e => {
                    e.preventDefault();
                    e.stopPropagation();
                    const row = btn.closest('.toc-row');
                    const href = hrefOf(row);
                    const expanded = directChildren(row).some(c => c.classList.contains('visible'));
                    if (expanded) {
                        allDescendants(row).forEach(conceal);
                        pinned.set(href, false);
                    } else {
                        directChildren(row).forEach(reveal);
                        pinned.set(href, true);
                    }
                    syncToggles();
                });
            });

            return { activate };
        }

        const tocs = Array.from(document.querySelectorAll('.toc'))
            .map(setupToc)
            .filter(Boolean);

        function activateAll(id) { tocs.forEach(t => t.activate(id)); }


        // Only observe chanes on desktop. It is annoying on mobile
        // if (window.innerWidth > 900) {
            const observer = new IntersectionObserver(entries => {
                for (const e of entries) {
                    if (e.isIntersecting) activateAll(e.target.id);
                }
            }, { rootMargin: '0px 0px -70% 0px' });

            headings.forEach(h => observer.observe(h));
        // }
    }

    if (document.readyState === 'complete') {
        setTimeout(init, 50);
    } else {
        window.addEventListener('load', () => setTimeout(init, 50));
    }
"##;

#[component]
pub fn BlogPost(id: String) -> Element {
    use_effect(|| {
        document::eval(SCROLL_OBSERVER_JS);
    });

    let post = use_server_future(move || {
        let id = id.clone();
        async move { get_post(id).await }
    })?;

    match &*post.read() {
        Some(Ok(data)) => rsx! {
            div {
                class: "blog-layout",
                TableOfContents {
                    entries: data.toc.clone(),
                    modifier: "toc-sidebar".to_string(),
                    id: "toc-sidebar".to_string(),
                }
                article {
                    class: "page-content blog-content",
                    h1 { "{data.meta.title}" }
                    div { class: "post-meta", "Toby Davis · {data.meta.date} · {data.meta.reading_time_minutes} min read" }
                    TableOfContents {
                        entries: data.toc.clone(),
                        modifier: "toc-card".to_string(),
                        id: "toc-card".to_string(),
                    }
                    div {
                        class: "post-body-wrap",
                        div { dangerous_inner_html: "{data.html}" }
                    }
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
