use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMeta {
    #[serde(default)]
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

fn folder_id(path: &std::path::Path) -> Option<String> {
    path.file_name()?.to_str()?.split_once('_').map(|(_, id)| id.to_string())
}

fn load_meta(dir: &std::path::Path) -> Option<PostMeta> {
    let id = folder_id(dir)?;
    let toml_str = std::fs::read_to_string(dir.join("config.toml")).ok()?;
    let mut meta: PostMeta = toml::from_str(&toml_str).ok()?;
    meta.id = id;
    Some(meta)
}

#[get("/api/posts")]
pub async fn get_all_posts() -> Result<Vec<PostMeta>> {
    let Ok(entries) = std::fs::read_dir(BLOG_DIR) else {
        return Ok(Vec::new());
    };
    let mut posts: Vec<PostMeta> = entries.flatten().filter_map(|e| load_meta(&e.path())).collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(posts)
}

#[get("/api/post/:id")]
pub async fn get_post(id: String) -> Result<PostData> {
    let post_dir = std::fs::read_dir(BLOG_DIR)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .flatten()
        .find(|e| folder_id(&e.path()).as_deref() == Some(&id))
        .map(|e| e.path())
        .ok_or_else(|| ServerFnError::new(format!("Post not found: {id}")))?;

    let meta = load_meta(&post_dir)
        .ok_or_else(|| ServerFnError::new(format!("Bad config for: {id}")))?;
    let md = std::fs::read_to_string(post_dir.join(&meta.entrypoint))
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let (html, toc) = render_markdown(&md);
    Ok(PostData { meta, html, toc })
}

// --- Server-only markdown rendering ---

#[cfg(feature = "server")]
fn render_markdown(markdown: &str) -> (String, Vec<TocEntry>) {
    use comrak::{Options, markdown_to_html};

    let mut opts = Options::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.tasklist = true;
    opts.render.r#unsafe = true;

    let raw = markdown_to_html(markdown, &opts);
    process_headings(&raw)
}

#[cfg(not(feature = "server"))]
fn render_markdown(_: &str) -> (String, Vec<TocEntry>) {
    (String::new(), Vec::new())
}

/// Single pass: insert id attributes on h2/h3 tags and extract ToC entries.
#[cfg(feature = "server")]
fn process_headings(html: &str) -> (String, Vec<TocEntry>) {
    let mut result = String::with_capacity(html.len());
    let mut toc = Vec::new();
    let mut rest = html;

    while let Some(pos) = rest.find("<h2>").or_else(|| rest.find("<h3>")) {
        let tag = if rest[pos..].starts_with("<h2>") { "h2" } else { "h3" };
        let level = if tag == "h2" { 2 } else { 3 };
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");

        result.push_str(&rest[..pos]);
        let after = pos + open.len();

        if let Some(end) = rest[after..].find(&close) {
            let raw_title = &rest[after..after + end];
            let title = strip_tags(raw_title);
            let id = slugify(&title);
            result.push_str(&format!("<{tag} id=\"{id}\">{raw_title}</{tag}>"));
            toc.push(TocEntry { id, title, level });
            rest = &rest[after + end + close.len()..];
        } else {
            result.push_str(&rest[pos..]);
            break;
        }
    }
    result.push_str(rest);
    (result, toc)
}

#[cfg(feature = "server")]
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
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
