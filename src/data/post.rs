use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Config read from each post's config.toml. The `id` is derived from the folder name, not stored in the file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMeta {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,
    #[serde(default = "default_entrypoint")]
    pub entrypoint: String,
    #[serde(default)]
    pub reading_time_minutes: u32,
}

fn default_entrypoint() -> String {
    "main.md".into()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostData {
    pub meta: PostMeta,
    pub html: String,
    pub toc: Vec<TocEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TocEntry {
    pub id: String,
    pub title: String,
    pub level: u8,
}

const BLOG_DIR: &str = "content/blog";

#[get("/api/posts")]
pub async fn get_all_posts() -> Result<Vec<PostMeta>> {
    let mut posts: Vec<PostMeta> = std::fs::read_dir(BLOG_DIR)
        .unwrap_or_else(|_| std::fs::read_dir("/dev/null").unwrap())
        .flatten()
        .filter_map(|entry| load_meta(&entry.path()))
        .collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(posts)
}

#[get("/api/post/:id")]
pub async fn get_post(id: String) -> Result<PostData> {
    let dir = std::path::Path::new(BLOG_DIR);

    // Find the folder that ends with the id (folders are <date>_<id>)
    let post_dir = std::fs::read_dir(dir)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .flatten()
        .find(|entry| folder_id(&entry.path()) == Some(id.clone()))
        .map(|e| e.path())
        .ok_or_else(|| ServerFnError::new(format!("Post not found: {id}")))?;

    let meta = load_meta(&post_dir)
        .ok_or_else(|| ServerFnError::new(format!("Bad config for: {id}")))?;

    let md = std::fs::read_to_string(post_dir.join(&meta.entrypoint))
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let (html, toc) = render_and_extract(&md);
    Ok(PostData { meta, html, toc })
}

/// Extract the post ID from a folder path like `2026-03-15_simd-math-library` → `simd-math-library`
fn folder_id(path: &std::path::Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    name.split_once('_').map(|(_, id)| id.to_string())
}

/// Load post metadata from a directory's config.toml, deriving the ID from the folder name.
fn load_meta(dir: &std::path::Path) -> Option<PostMeta> {
    let id = folder_id(dir)?;
    let toml_str = std::fs::read_to_string(dir.join("config.toml")).ok()?;

    #[derive(Deserialize)]
    struct RawConfig {
        title: String,
        description: String,
        date: String,
        #[serde(default = "default_entrypoint")]
        entrypoint: String,
        #[serde(default)]
        reading_time_minutes: u32,
    }

    let raw: RawConfig = toml::from_str(&toml_str).ok()?;
    Some(PostMeta {
        id,
        title: raw.title,
        description: raw.description,
        date: raw.date,
        entrypoint: raw.entrypoint,
        reading_time_minutes: raw.reading_time_minutes,
    })
}

/// Render markdown to HTML and extract ToC entries. Server-only.
#[cfg(feature = "server")]
fn render_and_extract(markdown: &str) -> (String, Vec<TocEntry>) {
    use comrak::{Options, markdown_to_html};

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.render.r#unsafe = true;

    let raw_html = markdown_to_html(markdown, &options);
    let html = add_heading_ids(&raw_html);
    let toc = extract_toc(&html);
    (html, toc)
}

#[cfg(not(feature = "server"))]
fn render_and_extract(_markdown: &str) -> (String, Vec<TocEntry>) {
    (String::new(), Vec::new())
}

/// Add id attributes to <h2> and <h3> tags for anchor links.
#[cfg(feature = "server")]
fn add_heading_ids(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(pos) = rest.find("<h2>").or_else(|| rest.find("<h3>")) {
        let tag = if rest[pos..].starts_with("<h2>") { "h2" } else { "h3" };
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");

        result.push_str(&rest[..pos]);
        let after = pos + open.len();

        if let Some(end) = rest[after..].find(&close) {
            let title = &rest[after..after + end];
            let id = slugify(title);
            result.push_str(&format!("<{tag} id=\"{id}\">{title}</{tag}>"));
            rest = &rest[after + end + close.len()..];
        } else {
            result.push_str(&rest[pos..]);
            break;
        }
    }
    result.push_str(rest);
    result
}

#[cfg(feature = "server")]
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Extract h2/h3 entries with id attributes from rendered HTML.
#[cfg(feature = "server")]
fn extract_toc(html: &str) -> Vec<TocEntry> {
    let mut entries = Vec::new();
    let mut rest = html;

    while let Some(pos) = rest.find("<h2 id=").or_else(|| rest.find("<h3 id=")) {
        let tag = if rest[pos..].starts_with("<h2") { "h2" } else { "h3" };
        let level = if tag == "h2" { 2 } else { 3 };
        let close = format!("</{tag}>");

        // Extract id value from id="..."
        if let Some(id_start) = rest[pos..].find("id=\"") {
            let id_begin = pos + id_start + 4;
            if let Some(id_end) = rest[id_begin..].find('"') {
                let id = rest[id_begin..id_begin + id_end].to_string();

                // Extract title text between > and </hN>
                if let Some(gt) = rest[pos..].find('>') {
                    let content_start = pos + gt + 1;
                    if let Some(close_pos) = rest[content_start..].find(&close) {
                        let title = strip_tags(&rest[content_start..content_start + close_pos]);
                        if !title.is_empty() {
                            entries.push(TocEntry { id, title, level });
                        }
                        rest = &rest[content_start + close_pos + close.len()..];
                        continue;
                    }
                }
            }
        }
        // Couldn't parse this heading, skip past it
        rest = &rest[pos + 4..];
    }
    entries
}

#[cfg(feature = "server")]
fn strip_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}
