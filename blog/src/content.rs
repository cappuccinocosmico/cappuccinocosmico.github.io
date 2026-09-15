use dioxus::prelude::*;
use site_content::models::{ContentItem, Recipe};

pub fn get_blogs() -> Vec<ContentItem> {
    site_content::get_blogs()
}

pub fn get_recipes() -> Vec<Recipe> {
    site_content::get_recipes()
}

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
