use crate::components::{Callout, CalloutKind, CodeBlock, TableOfContents, TocEntry};
use dioxus::prelude::*;

#[component]
pub fn BlogPost(slug: String) -> Element {
    let active_section = use_signal(|| "introduction".to_string());

    let toc_entries = vec![
        TocEntry { id: "introduction".into(), title: "Introduction".into(), level: 2 },
        TocEntry { id: "abstraction-layer".into(), title: "The Abstraction Layer".into(), level: 2 },
        TocEntry { id: "trait-design".into(), title: "Trait Design".into(), level: 3 },
        TocEntry { id: "backends".into(), title: "Backend Implementations".into(), level: 3 },
        TocEntry { id: "benchmarks".into(), title: "Benchmarks".into(), level: 2 },
        TocEntry { id: "conclusion".into(), title: "Conclusion".into(), level: 2 },
    ];

    rsx! {
        div {
            class: "blog-layout",

            TableOfContents {
                entries: toc_entries,
                active_id: active_section(),
            }

            article {
                class: "blog-content",

                h1 { "Building a SIMD-Accelerated Math Library" }
                div {
                    class: "post-meta",
                    "Toby Davis · March 2026 · 14 min read"
                }

                h2 { id: "introduction", "Introduction" }
                p {
                    "Modern CPUs ship with powerful SIMD instruction sets — AVX2, AVX-512, NEON — but writing portable vectorized code remains painful."
                }

                h2 { id: "abstraction-layer", "The Abstraction Layer" }
                p {
                    "The key insight is that most SIMD operations share the same semantics across architectures — they differ only in syntax."
                }

                h3 { id: "trait-design", "Trait Design" }
                p { "We define a trait that captures shared behaviour:" }

                Callout {
                    kind: CalloutKind::Note,
                    "All benchmarks were run on an M2 MacBook Pro with NEON enabled."
                }

                CodeBlock {
                    language: "rust".to_string(),
                    code: "pub trait SimdVector: Sized {{\n    fn splat(val: f32) -> Self;\n    fn fmadd(self, b: Self, c: Self) -> Self;\n}}".to_string(),
                }

                h3 { id: "backends", "Backend Implementations" }
                p { "Each architecture gets its own zero-cost implementation." }

                Callout {
                    kind: CalloutKind::Tip,
                    "Compile with "
                    code { "RUSTFLAGS=\"-C target-cpu=native\"" }
                    " for best results."
                }

                Callout {
                    kind: CalloutKind::Warning,
                    "Mixing AVX and SSE code without "
                    code { "_mm256_zeroupper" }
                    " causes severe performance penalties."
                }

                h2 { id: "benchmarks", "Benchmarks" }
                p { "Results across all three backends:" }

                Callout {
                    kind: CalloutKind::Success,
                    "All 847 tests passing across x86_64, aarch64, and scalar fallback."
                }

                h2 { id: "conclusion", "Conclusion" }
                p { "SIMD abstraction layers can provide zero-cost portability when designed carefully." }
            }

            div { class: "blog-spacer" }
        }
    }
}
