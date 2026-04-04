use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: String,
    pub description: String,
    pub tech: String,
    pub url: String,
}

pub fn get_projects() -> Vec<ProjectMeta> {
    vec![
        ProjectMeta {
            name: "LibRapid".to_string(),
            description: "High-performance math library with SIMD acceleration, lazy evaluation, and GPU compute support.".to_string(),
            tech: "C++ · CUDA · AVX2/512 · NEON".to_string(),
            url: "https://github.com/LibRapid/librapid".to_string(),
        },
        ProjectMeta {
            name: "tobydavis.dev".to_string(),
            description: "This website — a fullstack Rust application built with Dioxus.".to_string(),
            tech: "Rust · Dioxus · CSS".to_string(),
            url: "https://github.com/Pencilcaseman/tobydavis.dev".to_string(),
        },
    ]
}
