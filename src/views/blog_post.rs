use crate::components::TableOfContents;
use crate::data::get_post;
use dioxus::prelude::*;

const SCROLL_OBSERVER_JS: &str = r##"
    function init() {
        const headings = document.querySelectorAll('.blog-content [id]');
        const rows = Array.from(document.querySelectorAll('.toc-row'));
        if (!headings.length || !rows.length) return;

        const pinned = new Map(); // href -> bool

        // --- Helpers ---

        function itemOf(r) { return r.querySelector('.toc-item'); }
        function levelOf(r) { return parseInt(itemOf(r)?.dataset.level || '0'); }
        function hrefOf(r) { return itemOf(r)?.getAttribute('href')?.slice(1) || ''; }
        function isChild(r) { return r.classList.contains('child-row'); }

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

        // --- Visibility (row shown/hidden) ---

        function reveal(row) { row.classList.add('visible'); }
        function conceal(row) { row.classList.remove('visible'); }

        // --- Toggle sync (computed from DOM, never set speculatively) ---

        function syncToggles() {
            rows.forEach(row => {
                const btn = row.querySelector('.toc-toggle');
                if (!btn) return;
                const hasVisibleChild = directChildren(row).some(c => c.classList.contains('visible'));
                btn.classList.toggle('expanded', hasVisibleChild);
            });
        }

        // --- Core: activate a heading ---

        function activate(id) {
            // 1. Clear highlights
            rows.forEach(r => itemOf(r)?.classList.remove('active'));

            // 2. Collapse all child rows (except pinned)
            rows.filter(isChild).forEach(r => {
                if (!pinned.get(hrefOf(r))) conceal(r);
            });

            // 3. Find and highlight the active row
            const link = document.querySelector(`.toc-item[href="#${id}"]`);
            if (!link) { syncToggles(); return; }
            link.classList.add('active');
            const activeRow = link.closest('.toc-row');
            if (!activeRow) { syncToggles(); return; }

            // 4. Reveal the path from root to active:
            //    For each ancestor, show all siblings (= parent's direct children)
            const chain = ancestorChain(activeRow);
            for (const row of chain) {
                reveal(row);
                const p = parent(row);
                if (p) {
                    directChildren(p).forEach(reveal);
                }
            }

            // 5. Expand one level below the active item
            directChildren(activeRow).forEach(reveal);

            // 6. Re-expand pinned sections
            pinned.forEach((open, href) => {
                if (!open) return;
                const el = document.querySelector(`.toc-item[href="#${href}"]`)?.closest('.toc-row');
                if (el) directChildren(el).forEach(reveal);
            });

            // 7. Sync all toggle arrows from actual state
            syncToggles();
        }

        // --- Toggle click ---

        document.querySelectorAll('.toc-toggle').forEach(btn => {
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

        // --- Scroll observer ---

        const observer = new IntersectionObserver(entries => {
            for (const e of entries) {
                if (e.isIntersecting) activate(e.target.id);
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
