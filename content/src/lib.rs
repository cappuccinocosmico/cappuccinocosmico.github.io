use std::fs;
use std::path::{Path, PathBuf};

use num_rational::Rational64;
use pulldown_cmark::{Parser, html};
use serde::Deserialize;

pub mod models;

#[cfg(feature = "embed")]
pub mod embed;

pub fn content_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn blogs_dir() -> PathBuf {
    content_dir().join("blog")
}

pub fn recipes_dir() -> PathBuf {
    content_dir().join("recipies")
}

pub fn get_blogs() -> Vec<models::ContentItem> {
    collect_markdown(&blogs_dir())
        .into_iter()
        .map(|path| {
            let slug = slug_of(&path);
            let markdown = read_markdown(&path);
            parse_blog(&markdown, &slug)
        })
        .collect()
}

pub fn get_recipes() -> Vec<models::Recipe> {
    collect_markdown(&recipes_dir())
        .into_iter()
        .filter_map(|path| {
            let slug = slug_of(&path);
            let markdown = read_markdown(&path);
            parse_recipe(&markdown, &slug)
        })
        .collect()
}

pub fn parse_blog(markdown: &str, slug: &str) -> models::ContentItem {
    assert!(!slug.is_empty(), "blog slugs must not be empty");
    let (title, body) = split_frontmatter(markdown, slug);
    models::ContentItem {
        slug: slug.to_string(),
        title,
        html: markdown_to_html(&body),
    }
}

pub fn parse_recipe(markdown: &str, slug: &str) -> Option<models::Recipe> {
    assert!(!slug.is_empty(), "recipe slugs must not be empty");
    let (title, body) = split_frontmatter(markdown, slug);

    let mut ingredients = Vec::new();
    let mut instructions_md = String::new();
    let mut history = Vec::new();

    let mut current_section = "";
    let mut current_history_entry: Option<(String, String, String)> = None;
    let mut yaml_buffer = String::new();
    let mut in_yaml_block = false;

    for line in body.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("## Ingredients") {
            current_section = "ingredients";
            continue;
        } else if trimmed.starts_with("## Instructions") {
            if !yaml_buffer.is_empty() {
                if let Ok(parsed) = serde_yaml::from_str::<Vec<YamlIngredient>>(&yaml_buffer) {
                    ingredients = parsed.into_iter().map(|i| models::Ingredient {
                        qty: i.qty.and_then(|s| parse_quantity(&s)),
                        unit: i.unit.unwrap_or_else(|| "g".to_string()),
                        name: i.name,
                        note: i.note.unwrap_or_default(),
                    }).collect();
                }
                yaml_buffer.clear();
                in_yaml_block = false;
            }
            current_section = "instructions";
            continue;
        } else if trimmed.starts_with("## History") {
            current_section = "history";
            continue;
        } else if trimmed.starts_with("###") && current_section == "history" {
            if let Some((date, title, notes)) = current_history_entry.take() {
                let notes_html = markdown_to_html(&notes);
                history.push(models::HistoryEntry { date, title, notes_html });
            }

            let header = trimmed.trim_start_matches("###").trim();
            if let Some(dash_pos) = header.find(" - ") {
                let date = header[..dash_pos].trim().to_string();
                let title = header[dash_pos + 3..].trim().to_string();
                current_history_entry = Some((date, title, String::new()));
            }
            continue;
        }

        match current_section {
            "ingredients" => {
                if trimmed.starts_with("```yaml") {
                    in_yaml_block = true;
                    continue;
                } else if trimmed.starts_with("```") && in_yaml_block {
                    in_yaml_block = false;
                    continue;
                }

                if in_yaml_block {
                    yaml_buffer.push_str(line);
                    yaml_buffer.push('\n');
                }
            }
            "instructions" => {
                if !trimmed.is_empty() || !instructions_md.is_empty() {
                    instructions_md.push_str(line);
                    instructions_md.push('\n');
                }
            }
            "history" => {
                if let Some((_, _, ref mut notes)) = current_history_entry {
                    notes.push_str(line);
                    notes.push('\n');
                }
            }
            _ => {}
        }
    }

    if let Some((date, title, notes)) = current_history_entry {
        let notes_html = markdown_to_html(&notes);
        history.push(models::HistoryEntry { date, title, notes_html });
    }

    let instructions_html = markdown_to_html(&instructions_md);

    Some(models::Recipe {
        slug: slug.to_string(),
        title,
        ingredients,
        instructions_html,
        history,
    })
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YamlIngredient {
    qty: Option<String>,
    unit: Option<String>,
    name: String,
    note: Option<String>,
}

fn collect_markdown(dir: &Path) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("content directory {} must be readable: {err}", dir.display()))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    paths.sort_by_key(|path| slug_of(path));
    paths
}

fn read_markdown(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("content file {} must be readable: {err}", path.display()))
}

fn slug_of(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("untitled")
        .to_string()
}

fn split_frontmatter(content: &str, default_slug: &str) -> (String, String) {
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

    (slug_to_title(default_slug), content.to_string())
}

fn parse_quantity(s: &str) -> Option<Rational64> {
    let s = s.trim();

    if let Some(space_pos) = s.find(' ') {
        let whole_part = s[..space_pos].trim();
        let fraction_part = s[space_pos + 1..].trim();

        let whole: i64 = whole_part.parse().ok()?;
        let frac = parse_simple_fraction(fraction_part)?;

        return Some(Rational64::from_integer(whole) + frac);
    }

    if s.contains('/') {
        return parse_simple_fraction(s);
    }

    if let Ok(n) = s.parse::<i64>() {
        return Some(Rational64::from_integer(n));
    }

    None
}

fn parse_simple_fraction(s: &str) -> Option<Rational64> {
    let parts: Vec<&str> = s.split('/').collect();
    assert!(parts.len() <= 2, "fraction must have at most one '/' character");

    if parts.len() != 2 {
        return None;
    }

    let numer: i64 = parts[0].trim().parse().ok()?;
    let denom: i64 = parts[1].trim().parse().ok()?;
    assert!(denom != 0, "fraction denominator cannot be zero");

    Some(Rational64::new(numer, denom))
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
