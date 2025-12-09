use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: Option<String>,
    date: Option<String>,
}

struct ContentItem {
    slug: String,
    title: String,
    html: String,
}

fn main() {
    println!("cargo:rerun-if-changed=content/");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_content.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    // Process blogs
    let blogs = process_directory("content/blog");

    // Process recipes
    let recipes = process_directory("content/recipies");

    // Generate Rust code
    writeln!(f, "#[derive(Debug, Clone, PartialEq)]").unwrap();
    writeln!(f, "pub struct ContentItem {{").unwrap();
    writeln!(f, "    pub slug: &'static str,").unwrap();
    writeln!(f, "    pub title: &'static str,").unwrap();
    writeln!(f, "    pub html: &'static str,").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f).unwrap();

    // Generate blogs array
    writeln!(f, "pub static BLOGS: &[ContentItem] = &[").unwrap();
    for item in blogs {
        writeln!(f, "    ContentItem {{").unwrap();
        writeln!(f, "        slug: \"{}\",", escape_string(&item.slug)).unwrap();
        writeln!(f, "        title: \"{}\",", escape_string(&item.title)).unwrap();
        writeln!(f, "        html: r#\"{}\"#,", item.html).unwrap();
        writeln!(f, "    }},").unwrap();
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    // Generate recipes array
    writeln!(f, "pub static RECIPES: &[ContentItem] = &[").unwrap();
    for item in recipes {
        writeln!(f, "    ContentItem {{").unwrap();
        writeln!(f, "        slug: \"{}\",", escape_string(&item.slug)).unwrap();
        writeln!(f, "        title: \"{}\",", escape_string(&item.title)).unwrap();
        writeln!(f, "        html: r#\"{}\"#,", item.html).unwrap();
        writeln!(f, "    }},").unwrap();
    }
    writeln!(f, "];").unwrap();
}

fn process_directory(dir: &str) -> Vec<ContentItem> {
    let mut items = Vec::new();

    let path = Path::new(dir);
    if !path.exists() {
        return items;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return items,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "md" {
                continue;
            }
        } else {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string();

        let (title, markdown) = parse_content(&content, &slug);
        let html = markdown_to_html(&markdown);

        items.push(ContentItem { slug, title, html });
    }

    items.sort_by(|a, b| a.slug.cmp(&b.slug));
    items
}

fn parse_content(content: &str, default_slug: &str) -> (String, String) {
    // Check if content starts with YAML frontmatter
    if content.starts_with("---") {
        if let Some(end_pos) = content[3..].find("---") {
            let frontmatter_str = &content[3..end_pos + 3];
            let markdown = &content[end_pos + 6..];

            if let Ok(frontmatter) = serde_yaml::from_str::<Frontmatter>(frontmatter_str) {
                if let Some(title) = frontmatter.title {
                    return (title, markdown.to_string());
                }
            }
        }
    }

    // No frontmatter or no title in frontmatter, use filename
    let title = slug_to_title(default_slug);
    (title, content.to_string())
}

fn slug_to_title(slug: &str) -> String {
    let skip_words = ["and", "or", "the", "a", "an", "of", "in", "on", "at", "to", "for"];

    slug.split('-')
        .map(|word| {
            if skip_words.contains(&word) {
                word.to_string()
            } else {
                capitalize_word(word)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
