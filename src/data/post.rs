use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostConfig {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub date: String,
    pub entrypoint: String,
    pub reading_time_minutes: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub date: String,
    pub reading_time_minutes: u32,
}

#[get("/api/posts")]
pub async fn get_all_posts() -> Result<Vec<PostMeta>> {
    let content_dir = std::path::Path::new("content/blog");
    let mut posts = Vec::new();

    if let Ok(entries) = std::fs::read_dir(content_dir) {
        for entry in entries.flatten() {
            let config_path = entry.path().join("config.toml");
            if config_path.exists() {
                if let Ok(contents) = std::fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str::<PostConfig>(&contents) {
                        posts.push(PostMeta {
                            title: config.title,
                            slug: config.slug,
                            description: config.description,
                            date: config.date,
                            reading_time_minutes: config.reading_time_minutes,
                        });
                    }
                }
            }
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(posts)
}
