use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct YamlIngredient {
    qty: Option<f64>,
    unit: Option<String>,
    name: String,
    note: Option<String>,
}

struct ContentItem {
    slug: String,
    title: String,
    html: String,
}

struct Recipe {
    slug: String,
    title: String,
    ingredients: Vec<Ingredient>,
    instructions_html: String,
    history: Vec<HistoryEntry>,
}

struct Ingredient {
    qty: Option<f64>,
    unit: String,
    name: String,
    note: String,
}

struct HistoryEntry {
    date: String,
    title: String,
    notes_html: String,
}

fn main() {
    println!("cargo:rerun-if-changed=content/");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_content.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    // Process blogs (unchanged)
    let blogs = process_blog_directory("content/blog");

    // Process recipes (new structured format)
    let recipes = process_recipe_directory("content/recipies");

    // Generate ContentItem struct for blogs
    writeln!(f, "#[derive(Debug, Clone, PartialEq)]").unwrap();
    writeln!(f, "pub struct ContentItem {{").unwrap();
    writeln!(f, "    pub slug: &'static str,").unwrap();
    writeln!(f, "    pub title: &'static str,").unwrap();
    writeln!(f, "    pub html: &'static str,").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f).unwrap();

    // Generate Ingredient struct
    writeln!(f, "#[derive(Debug, Clone, PartialEq)]").unwrap();
    writeln!(f, "pub struct Ingredient {{").unwrap();
    writeln!(f, "    pub qty: Option<f64>,").unwrap();
    writeln!(f, "    pub unit: &'static str,").unwrap();
    writeln!(f, "    pub name: &'static str,").unwrap();
    writeln!(f, "    pub note: &'static str,").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f).unwrap();

    // Generate Recipe struct
    writeln!(f, "#[derive(Debug, Clone, PartialEq)]").unwrap();
    writeln!(f, "pub struct Recipe {{").unwrap();
    writeln!(f, "    pub slug: &'static str,").unwrap();
    writeln!(f, "    pub title: &'static str,").unwrap();
    writeln!(f, "    pub ingredients: &'static [Ingredient],").unwrap();
    writeln!(f, "    pub instructions_html: &'static str,").unwrap();
    writeln!(f, "    pub history: &'static [HistoryEntry],").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f).unwrap();

    // Generate HistoryEntry struct
    writeln!(f, "#[derive(Debug, Clone, PartialEq)]").unwrap();
    writeln!(f, "pub struct HistoryEntry {{").unwrap();
    writeln!(f, "    pub date: &'static str,").unwrap();
    writeln!(f, "    pub title: &'static str,").unwrap();
    writeln!(f, "    pub notes_html: &'static str,").unwrap();
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
    writeln!(f, "pub static RECIPES: &[Recipe] = &[").unwrap();
    for recipe in recipes {
        writeln!(f, "    Recipe {{").unwrap();
        writeln!(f, "        slug: \"{}\",", escape_string(&recipe.slug)).unwrap();
        writeln!(f, "        title: \"{}\",", escape_string(&recipe.title)).unwrap();

        // Write ingredients array
        writeln!(f, "        ingredients: &[").unwrap();
        for ingredient in &recipe.ingredients {
            writeln!(f, "            Ingredient {{").unwrap();
            if let Some(qty) = ingredient.qty {
                writeln!(f, "                qty: Some({}),", qty).unwrap();
            } else {
                writeln!(f, "                qty: None,").unwrap();
            }
            writeln!(f, "                unit: \"{}\",", escape_string(&ingredient.unit)).unwrap();
            writeln!(f, "                name: \"{}\",", escape_string(&ingredient.name)).unwrap();
            writeln!(f, "                note: \"{}\",", escape_string(&ingredient.note)).unwrap();
            writeln!(f, "            }},").unwrap();
        }
        writeln!(f, "        ],").unwrap();

        writeln!(f, "        instructions_html: r#\"{}\"#,", recipe.instructions_html).unwrap();

        // Write history array
        writeln!(f, "        history: &[").unwrap();
        for entry in &recipe.history {
            writeln!(f, "            HistoryEntry {{").unwrap();
            writeln!(f, "                date: \"{}\",", escape_string(&entry.date)).unwrap();
            writeln!(f, "                title: \"{}\",", escape_string(&entry.title)).unwrap();
            writeln!(f, "                notes_html: r#\"{}\"#,", entry.notes_html).unwrap();
            writeln!(f, "            }},").unwrap();
        }
        writeln!(f, "        ],").unwrap();

        writeln!(f, "    }},").unwrap();
    }
    writeln!(f, "];").unwrap();
}

fn process_blog_directory(dir: &str) -> Vec<ContentItem> {
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

fn process_recipe_directory(dir: &str) -> Vec<Recipe> {
    let mut recipes = Vec::new();

    let path = Path::new(dir);
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
                        qty: i.qty,
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
