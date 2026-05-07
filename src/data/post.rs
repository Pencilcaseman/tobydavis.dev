#[cfg(feature = "server")]
use std::path::PathBuf;

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

fn default_entrypoint() -> String {
    "main.typ".into()
}

#[cfg(feature = "server")]
fn blog_root() -> PathBuf {
    let args = crate::misc::cli::get();
    args.content_root.clone().unwrap_or("content".into()).join("blog")
}

#[cfg(feature = "server")]
fn folder_id(path: &std::path::Path) -> Option<String> {
    path.file_name()?.to_str()?.split_once('_').map(|(_, id)| id.to_string())
}

#[cfg(feature = "server")]
fn load_meta(dir: &std::path::Path) -> Option<PostMeta> {
    let id = folder_id(dir)?;
    let toml_str = std::fs::read_to_string(dir.join("config.toml")).ok()?;
    let mut meta: PostMeta = toml::from_str(&toml_str).ok()?;
    meta.id = id;

    Some(meta)
}

#[get("/api/posts")]
pub async fn get_all_posts() -> Result<Vec<PostMeta>> {
    let Ok(entries) = std::fs::read_dir(blog_root()) else {
        dioxus::logger::tracing::warn!("Blog post directory empty");
        return Ok(Vec::new());
    };

    let mut posts: Vec<PostMeta> =
        entries.flatten().filter_map(|e| load_meta(&e.path())).collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

#[get("/api/post/:id")]
pub async fn get_post(id: String) -> Result<PostData> {
    let post_dir = std::fs::read_dir(blog_root())
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .flatten()
        .find(|e| folder_id(&e.path()).as_deref() == Some(&id))
        .map(|e| e.path())
        .ok_or_else(|| ServerFnError::new(format!("Post not found: {id}")))?;

    let meta = load_meta(&post_dir)
        .ok_or_else(|| ServerFnError::new(format!("Bad config for: {id}")))?;

    #[cfg(feature = "server")]
    {
        let hash = hash_post_inputs(&post_dir).map_err(|e| {
            ServerFnError::new(format!("Failed to hash post inputs: {e}"))
        })?;

        if let Some(entry) = POST_CACHE.get(&id) {
            if entry.hash == hash {
                return Ok(entry.data);
            }
        }

        let (html, toc) = render_post(&post_dir, &meta.entrypoint)?;
        let data = PostData { meta, html, toc };
        POST_CACHE.insert(id, CacheEntry { hash, data: data.clone() });
        return Ok(data);
    }

    #[cfg(not(feature = "server"))]
    {
        let (html, toc) = render_post(&post_dir, &meta.entrypoint)?;
        Ok(PostData { meta, html, toc })
    }
}

#[cfg(feature = "server")]
#[derive(Clone)]
struct CacheEntry {
    hash: u64,
    data: PostData,
}

#[cfg(feature = "server")]
static POST_CACHE: std::sync::LazyLock<moka::sync::Cache<String, CacheEntry>> =
    std::sync::LazyLock::new(|| {
        moka::sync::Cache::builder()
            .max_capacity(128)
            .time_to_live(std::time::Duration::from_secs(60 * 60))
            .time_to_idle(std::time::Duration::from_secs(15 * 60))
            .build()
    });

#[cfg(feature = "server")]
fn hash_post_inputs(post_dir: &std::path::Path) -> std::io::Result<u64> {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let mut hasher = DefaultHasher::new();

    let template_path = std::path::Path::new("content/template.typ");
    if template_path.exists() {
        std::fs::read(template_path)?.hash(&mut hasher);
    }

    let mut entries: Vec<_> = std::fs::read_dir(post_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        entry.file_name().hash(&mut hasher);
        std::fs::read(entry.path())?.hash(&mut hasher);
    }

    Ok(hasher.finish())
}

#[cfg(feature = "server")]
fn render_post(
    post_dir: &std::path::Path,
    entrypoint: &str,
) -> std::result::Result<(String, Vec<TocEntry>), ServerFnError> {
    use std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    };

    use typst::{
        Feature, Library, LibraryExt, World,
        diag::{FileError, FileResult},
        foundations::{Bytes, Datetime},
        syntax::{FileId, Source, VirtualPath},
        text::{Font, FontBook},
        utils::LazyHash,
    };
    use typst_kit::{
        download::{Downloader, ProgressSink},
        package::PackageStorage,
    };

    static FONTS: LazyLock<(LazyHash<FontBook>, Vec<Font>)> =
        LazyLock::new(|| {
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

    static PACKAGES: LazyLock<PackageStorage> = LazyLock::new(|| {
        let ua =
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        PackageStorage::new(None, None, Downloader::new(ua))
    });

    struct BlogWorld {
        main: FileId,
        sources: Mutex<HashMap<FileId, Source>>,
        files: Mutex<HashMap<FileId, Bytes>>,
    }

    impl BlogWorld {
        fn read_package_file(&self, id: FileId) -> FileResult<Vec<u8>> {
            let spec = id.package().ok_or_else(|| {
                FileError::NotFound(id.vpath().as_rootless_path().into())
            })?;
            let pkg_root = PACKAGES
                .prepare_package(spec, &mut ProgressSink)
                .map_err(|e| FileError::Package(e))?;
            let path = id.vpath().resolve(&pkg_root).ok_or_else(|| {
                FileError::NotFound(id.vpath().as_rootless_path().into())
            })?;
            std::fs::read(&path).map_err(|e| FileError::from_io(e, &path))
        }
    }

    impl World for BlogWorld {
        fn library(&self) -> &LazyHash<Library> {
            &LIBRARY
        }
        fn book(&self) -> &LazyHash<FontBook> {
            &FONTS.0
        }
        fn main(&self) -> FileId {
            self.main
        }

        fn source(&self, id: FileId) -> FileResult<Source> {
            if let Some(s) = self.sources.lock().unwrap().get(&id) {
                return Ok(s.clone());
            }
            if id.package().is_some() {
                let bytes = self.read_package_file(id)?;
                let text = String::from_utf8(bytes)
                    .map_err(|_| FileError::InvalidUtf8)?;
                let source = Source::new(id, text);
                self.sources.lock().unwrap().insert(id, source.clone());
                return Ok(source);
            }
            Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
        }

        fn file(&self, id: FileId) -> FileResult<Bytes> {
            if let Some(b) = self.files.lock().unwrap().get(&id) {
                return Ok(b.clone());
            }
            if id.package().is_some() {
                let bytes = Bytes::new(self.read_package_file(id)?);
                self.files.lock().unwrap().insert(id, bytes.clone());
                return Ok(bytes);
            }
            Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
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

    let content_root = std::path::Path::new("content");
    let main_path = post_dir.join(entrypoint);
    let main_text = std::fs::read_to_string(&main_path).map_err(|e| {
        ServerFnError::new(format!(
            "Failed to read {}: {e}",
            main_path.display()
        ))
    })?;

    let rel_main = main_path.strip_prefix(content_root).unwrap_or(&main_path);
    let main_id = FileId::new(None, VirtualPath::new(rel_main));

    let mut sources = HashMap::new();
    let mut files: HashMap<FileId, Bytes> = HashMap::new();

    sources.insert(main_id, Source::new(main_id, main_text));

    // Load template
    let template_path = content_root.join("template.typ");
    if template_path.exists() {
        let text = std::fs::read_to_string(&template_path).map_err(|e| {
            ServerFnError::new(format!("Failed to read template: {e}"))
        })?;
        let id = FileId::new(None, VirtualPath::new("template.typ"));
        sources.insert(id, Source::new(id, text));
    }

    if let Ok(entries) = std::fs::read_dir(post_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext =
                    path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext != "typ" && ext != "toml" {
                    if let Ok(data) = std::fs::read(&path) {
                        let rel =
                            path.strip_prefix(content_root).unwrap_or(&path);
                        let fid = FileId::new(None, VirtualPath::new(rel));
                        files.insert(fid, Bytes::new(data));
                    }
                }
            }
        }
    }

    let world = BlogWorld {
        main: main_id,
        sources: Mutex::new(sources),
        files: Mutex::new(files),
    };

    // Compile
    let result = typst::compile::<typst_html::HtmlDocument>(&world);
    let doc = result.output.map_err(|errs| {
        let msgs: Vec<String> = errs
            .iter()
            .map(|e| format!("{}: {:?}", e.message, e.span))
            .collect();
        ServerFnError::new(format!("Typst error:\n{}", msgs.join("\n")))
    })?;

    let full_html = typst_html::html(&doc).map_err(|errs| {
        let msgs: Vec<String> =
            errs.iter().map(|e| format!("{}", e.message)).collect();
        ServerFnError::new(format!("HTML export error:\n{}", msgs.join("\n")))
    })?;

    // Make Typst's hardcoded black fills/strokes follow the page's text color
    // so equations theme correctly in light/dark mode.
    let full_html = full_html
        .replace(r##"fill="#000000""##, r##"fill="currentColor""##)
        .replace(r##"fill="#000""##, r##"fill="currentColor""##)
        .replace(r##"stroke="#000000""##, r##"stroke="currentColor""##)
        .replace(r##"stroke="#000""##, r##"stroke="currentColor""##);

    let body = extract_body(&full_html);
    let (processed, toc) = process_headings(&body);
    Ok((processed, toc))
}

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
