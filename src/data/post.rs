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

/// Full post data returned by the server: metadata + rendered HTML + ToC entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostData {
    pub meta: PostMeta,
    pub html: String,
    pub toc: Vec<TocData>,
}

/// A table-of-contents entry extracted from rendered HTML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TocData {
    pub id: String,
    pub title: String,
    pub level: u8,
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

#[get("/api/post/:slug")]
pub async fn get_post(slug: String) -> Result<PostData> {
    let content_dir = std::path::Path::new("content/blog");

    let entries = std::fs::read_dir(content_dir)
        .map_err(|e| ServerFnError::new(format!("Failed to read content dir: {e}")))?;

    for entry in entries.flatten() {
        let config_path = entry.path().join("config.toml");
        if !config_path.exists() {
            continue;
        }

        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| ServerFnError::new(format!("Failed to read config: {e}")))?;
        let config: PostConfig = toml::from_str(&config_str)
            .map_err(|e| ServerFnError::new(format!("Failed to parse config: {e}")))?;

        if config.slug != slug {
            continue;
        }

        let md_path = entry.path().join(&config.entrypoint);
        let markdown = std::fs::read_to_string(&md_path)
            .map_err(|e| ServerFnError::new(format!("Failed to read markdown: {e}")))?;

        let html = render_markdown(&markdown);
        let toc = extract_toc(&html);

        return Ok(PostData {
            meta: PostMeta {
                title: config.title,
                slug: config.slug,
                description: config.description,
                date: config.date,
                reading_time_minutes: config.reading_time_minutes,
            },
            html,
            toc,
        });
    }

    Err(ServerFnError::new(format!("Post not found: {slug}")))
}

#[cfg(feature = "server")]
fn render_markdown(markdown: &str) -> String {
    use comrak::{markdown_to_html, Options};

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.render.unsafe_ = true;

    let html = markdown_to_html(markdown, &options);

    // Post-process: add id attributes to h2 and h3 for anchor links
    add_heading_ids(&html)
}

#[cfg(not(feature = "server"))]
fn render_markdown(_markdown: &str) -> String {
    String::new()
}

#[cfg(feature = "server")]
fn add_heading_ids(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(tag_start) = rest.find("<h2>").or_else(|| rest.find("<h3>")) {
        let is_h2 = rest[tag_start..].starts_with("<h2>");
        let tag = if is_h2 { "h2" } else { "h3" };
        let open_tag = format!("<{tag}>");
        let close_tag = format!("</{tag}>");

        // Copy everything before this tag
        result.push_str(&rest[..tag_start]);

        let after_open = tag_start + open_tag.len();
        if let Some(close_pos) = rest[after_open..].find(&close_tag) {
            let title = &rest[after_open..after_open + close_pos];
            let id = slugify(title);
            result.push_str(&format!("<{tag} id=\"{id}\">{title}</{tag}>"));
            rest = &rest[after_open + close_pos + close_tag.len()..];
        } else {
            result.push_str(&rest[tag_start..]);
            break;
        }
    }
    result.push_str(rest);
    result
}

#[cfg(feature = "server")]
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(feature = "server")]
fn extract_toc(html: &str) -> Vec<TocData> {
    let mut entries = Vec::new();
    let mut rest = html.as_bytes();

    while let Some(pos) = find_heading_tag(rest) {
        let tag_slice = &rest[pos..];
        let is_h2 = tag_slice.starts_with(b"<h2");
        let level = if is_h2 { 2 } else { 3 };

        // Find the id attribute
        if let Some(id) = extract_attr(tag_slice, "id") {
            // Find the closing tag
            let close_tag = if is_h2 { b"</h2>" as &[u8] } else { b"</h3>" };
            if let Some(close_pos) = find_bytes(tag_slice, close_tag) {
                // Find where the opening tag ends
                if let Some(gt_pos) = find_bytes(tag_slice, b">") {
                    let title_bytes = &tag_slice[gt_pos + 1..close_pos];
                    let title = strip_html_tags(&String::from_utf8_lossy(title_bytes));
                    if !title.is_empty() {
                        entries.push(TocData {
                            id: id.to_string(),
                            title,
                            level,
                        });
                    }
                }
            }
        }

        // Advance past this tag
        rest = &rest[pos + 4..];
    }

    entries
}

#[cfg(feature = "server")]
fn find_heading_tag(haystack: &[u8]) -> Option<usize> {
    for i in 0..haystack.len().saturating_sub(3) {
        if (haystack[i..].starts_with(b"<h2") || haystack[i..].starts_with(b"<h3"))
            && (haystack.len() > i + 3 && (haystack[i + 3] == b' ' || haystack[i + 3] == b'>'))
        {
            return Some(i);
        }
    }
    None
}

#[cfg(feature = "server")]
fn extract_attr<'a>(tag: &'a [u8], attr_name: &str) -> Option<&'a str> {
    let tag_str = std::str::from_utf8(tag).ok()?;
    let gt_pos = tag_str.find('>')?;
    let open_tag = &tag_str[..gt_pos];
    let pattern = format!("{attr_name}=\"");
    let start = open_tag.find(&pattern)? + pattern.len();
    let end = open_tag[start..].find('"')? + start;
    Some(&open_tag[start..end])
}

#[cfg(feature = "server")]
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}

#[cfg(feature = "server")]
fn strip_html_tags(input: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in input.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result
}
