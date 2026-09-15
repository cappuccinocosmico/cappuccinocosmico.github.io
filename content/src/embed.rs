include!(concat!(env!("OUT_DIR"), "/embedded_content.rs"));

use crate::models::{ContentItem, Recipe};

pub fn embedded_blogs() -> Vec<ContentItem> {
    BLOG_SOURCES
        .iter()
        .map(|(slug, markdown)| crate::parse_blog(markdown, slug))
        .collect()
}

pub fn embedded_recipes() -> Vec<Recipe> {
    RECIPE_SOURCES
        .iter()
        .filter_map(|(slug, markdown)| crate::parse_recipe(markdown, slug))
        .collect()
}
