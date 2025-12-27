use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use num_rational::Rational64;
use crate::models::{ContentItem, Recipe, Ingredient, HistoryEntry};

#[cfg(feature = "server")]
use std::fs;
#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use server_fn::{ServerFnError, server};

#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Debug, Deserialize, Clone)]
struct YamlIngredient {
    qty: Option<String>,
    unit: Option<String>,
    name: String,
    note: Option<String>,
}

/// Parse all blog posts from content/blog directory (server-side only)
#[cfg(feature = "server")]
pub fn get_blogs() -> Vec<ContentItem> {
    let mut items = Vec::new();

    let path = Path::new("content/blog");
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

/// Parse all recipes from content/recipies directory (server-side only)
#[cfg(feature = "server")]
pub fn get_recipes() -> Vec<Recipe> {
    let mut recipes = Vec::new();

    let path = Path::new("content/recipies");
    if !path.exists() {
        return recipes;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return recipes,
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

        if let Some(recipe) = parse_recipe(&content, slug) {
            recipes.push(recipe);
        }
    }

    recipes.sort_by(|a, b| a.slug.cmp(&b.slug));
    recipes
}

#[cfg(feature = "server")]
fn parse_recipe(content: &str, slug: String) -> Option<Recipe> {
    let (title, body) = parse_content(content, &slug);

    // Split content into sections
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
            // Parse any accumulated YAML
            if !yaml_buffer.is_empty() {
                if let Ok(parsed) = serde_yaml::from_str::<Vec<YamlIngredient>>(&yaml_buffer) {
                    ingredients = parsed.into_iter().map(|i| Ingredient {
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
            // Save previous history entry if exists
            if let Some((date, title, notes)) = current_history_entry.take() {
                let notes_html = markdown_to_html(&notes);
                history.push(HistoryEntry { date, title, notes_html });
            }

            // Parse new history entry header: ### YYYY-MM-DD - Title
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
                // Check for YAML code fence
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

    // Don't forget the last history entry
    if let Some((date, title, notes)) = current_history_entry {
        let notes_html = markdown_to_html(&notes);
        history.push(HistoryEntry { date, title, notes_html });
    }

    let instructions_html = markdown_to_html(&instructions_md);

    Some(Recipe {
        slug,
        title,
        ingredients,
        instructions_html,
        history,
    })
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
fn parse_quantity(s: &str) -> Option<Rational64> {
    let s = s.trim();

    // Handle mixed fractions like "1 1/3"
    if let Some(space_pos) = s.find(' ') {
        let whole_part = s[..space_pos].trim();
        let fraction_part = s[space_pos + 1..].trim();

        let whole: i64 = whole_part.parse().ok()?;
        let frac = parse_simple_fraction(fraction_part)?;

        // Convert mixed fraction: whole + frac
        return Some(Rational64::from_integer(whole) + frac);
    }

    // Handle simple fractions like "4/3"
    if s.contains('/') {
        return parse_simple_fraction(s);
    }

    // Handle integers like "1"
    if let Ok(n) = s.parse::<i64>() {
        return Some(Rational64::from_integer(n));
    }

    None
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(feature = "server")]
fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

// Server functions for fetching content (only available on server side)
#[cfg(feature = "server")]
#[server(endpoint = "get_all_blogs")]
pub async fn get_all_blogs_server() -> Result<Vec<ContentItem>, ServerFnError> {
    Ok(get_blogs())
}

#[cfg(feature = "server")]
#[server(endpoint = "get_blog_by_slug")]
pub async fn get_blog_by_slug_server(slug: String) -> Result<ContentItem, ServerFnError> {
    let blogs = get_blogs();
    blogs.iter()
        .find(|b| b.slug == slug)
        .cloned()
        .ok_or_else(|| ServerFnError::new("Blog not found"))
}

#[cfg(feature = "server")]
#[server(endpoint = "get_all_recipes")]
pub async fn get_all_recipes_server() -> Result<Vec<Recipe>, ServerFnError> {
    Ok(get_recipes())
}

#[cfg(feature = "server")]
#[server(endpoint = "get_recipe_by_slug")]
pub async fn get_recipe_by_slug_server(slug: String) -> Result<Recipe, ServerFnError> {
    let recipes = get_recipes();
    recipes.iter()
        .find(|r| r.slug == slug)
        .cloned()
        .ok_or_else(|| ServerFnError::new("Recipe not found"))
}
