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
    "main.typ".into()
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

    let (html, toc) = render_post(&post_dir, &meta.entrypoint)?;
    Ok(PostData { meta, html, toc })
}

// --- Server-side Typst rendering ---

#[cfg(feature = "server")]
fn render_post(
    post_dir: &std::path::Path,
    entrypoint: &str,
) -> std::result::Result<(String, Vec<TocEntry>), ServerFnError> {
    use std::collections::HashMap;
    use std::sync::LazyLock;
    use typst::diag::{FileError, FileResult};
    use typst::foundations::{Bytes, Datetime};
    use typst::syntax::{FileId, Source, VirtualPath};
    use typst::text::{Font, FontBook};
    use typst::utils::LazyHash;
    use typst::{Feature, Library, LibraryExt, World};

    // Fonts are expensive — cache globally
    static FONTS: LazyLock<(LazyHash<FontBook>, Vec<Font>)> = LazyLock::new(|| {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();
        for data in typst_assets::fonts() {
            let bytes = Bytes::new(data);
            for font in Font::iter(bytes) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }
        (LazyHash::new(book), fonts)
    });

    static LIBRARY: LazyLock<LazyHash<Library>> = LazyLock::new(|| {
        let features = [Feature::Html].into_iter().collect();
        let lib = Library::builder().with_features(features).build();
        LazyHash::new(lib)
    });

    struct BlogWorld {
        main: FileId,
        sources: HashMap<FileId, Source>,
        files: HashMap<FileId, Bytes>,
    }

    impl World for BlogWorld {
        fn library(&self) -> &LazyHash<Library> { &LIBRARY }
        fn book(&self) -> &LazyHash<FontBook> { &FONTS.0 }
        fn main(&self) -> FileId { self.main }

        fn source(&self, id: FileId) -> FileResult<Source> {
            self.sources.get(&id).cloned().ok_or(FileError::NotFound(
                id.vpath().as_rootless_path().into(),
            ))
        }

        fn file(&self, id: FileId) -> FileResult<Bytes> {
            self.files.get(&id).cloned().ok_or(FileError::NotFound(
                id.vpath().as_rootless_path().into(),
            ))
        }

        fn font(&self, index: usize) -> Option<Font> {
            FONTS.1.get(index).cloned()
        }

        fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
            let now = chrono::Utc::now();
            use chrono::Datelike;
            Datetime::from_ymd(
                now.year(),
                now.month().try_into().ok()?,
                now.day().try_into().ok()?,
            )
        }
    }

    // Build the world
    let content_root = std::path::Path::new("content");
    let main_path = post_dir.join(entrypoint);
    let main_text = std::fs::read_to_string(&main_path)
        .map_err(|e| ServerFnError::new(format!("Failed to read {}: {e}", main_path.display())))?;

    let rel_main = main_path.strip_prefix(content_root).unwrap_or(&main_path);
    let main_id = FileId::new(None, VirtualPath::new(rel_main));

    let mut sources = HashMap::new();
    let mut files: HashMap<FileId, Bytes> = HashMap::new();

    sources.insert(main_id, Source::new(main_id, main_text));

    // Load template
    let template_path = content_root.join("template.typ");
    if template_path.exists() {
        let text = std::fs::read_to_string(&template_path)
            .map_err(|e| ServerFnError::new(format!("Failed to read template: {e}")))?;
        let id = FileId::new(None, VirtualPath::new("template.typ"));
        sources.insert(id, Source::new(id, text));
    }

    // Load non-source files from post directory (images, data, etc.)
    if let Ok(entries) = std::fs::read_dir(post_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext != "typ" && ext != "toml" {
                    if let Ok(data) = std::fs::read(&path) {
                        let rel = path.strip_prefix(content_root).unwrap_or(&path);
                        let fid = FileId::new(None, VirtualPath::new(rel));
                        files.insert(fid, Bytes::new(data));
                    }
                }
            }
        }
    }

    let world = BlogWorld { main: main_id, sources, files };

    // Compile
    let result = typst::compile::<typst_html::HtmlDocument>(&world);
    let doc = result.output.map_err(|errs| {
        let msgs: Vec<String> = errs.iter().map(|e| format!("{}: {:?}", e.message, e.span)).collect();
        ServerFnError::new(format!("Typst error:\n{}", msgs.join("\n")))
    })?;

    let full_html = typst_html::html(&doc).map_err(|errs| {
        let msgs: Vec<String> = errs.iter().map(|e| format!("{}", e.message)).collect();
        ServerFnError::new(format!("HTML export error:\n{}", msgs.join("\n")))
    })?;

    // Debug: find where SVGs are in the output
    for (i, _) in full_html.match_indices("<svg") {
        let end = (i + 200).min(full_html.len());
        eprintln!("=== SVG at byte {} ===\n{}\n", i, &full_html[i..end]);
    }
    // Also dump the full HTML length and first occurrence of "dot product"
    eprintln!("=== Full HTML length: {} ===", full_html.len());
    if let Some(idx) = full_html.find("dot product") {
        let start = idx.saturating_sub(50);
        let end = (idx + 500).min(full_html.len());
        eprintln!("=== AROUND 'dot product' ===\n{}\n=== END ===", &full_html[start..end]);
    }

    let body = extract_body(&full_html);
    let (processed, toc) = process_headings(&body);
    Ok((processed, toc))
}

#[cfg(not(feature = "server"))]
fn render_post(
    _post_dir: &std::path::Path,
    _entrypoint: &str,
) -> std::result::Result<(String, Vec<TocEntry>), ServerFnError> {
    Ok((String::new(), Vec::new()))
}

// --- HTML post-processing ---

#[cfg(feature = "server")]
fn extract_body(html: &str) -> String {
    let start = html.find("<body>").map(|i| i + 6).unwrap_or(0);
    let end = html.rfind("</body>").unwrap_or(html.len());
    html[start..end].to_string()
}

#[cfg(feature = "server")]
fn process_headings(html: &str) -> (String, Vec<TocEntry>) {
    let mut result = String::with_capacity(html.len());
    let mut toc = Vec::new();
    let mut rest = html;

    while let Some(pos) = find_next_heading(rest) {
        let level = (rest.as_bytes()[pos + 2] - b'0') as u8;
        let tag = format!("h{level}");
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
fn find_next_heading(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    for i in 0..bytes.len().saturating_sub(3) {
        if bytes[i] == b'<'
            && bytes[i + 1] == b'h'
            && (b'1'..=b'6').contains(&bytes[i + 2])
            && bytes[i + 3] == b'>'
        {
            return Some(i);
        }
    }
    None
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
