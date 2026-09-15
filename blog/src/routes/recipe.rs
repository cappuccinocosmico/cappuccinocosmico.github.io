use dioxus::prelude::*;
use crate::{Route, content};
use crate::components::recipe::{RecipeIngredients, RecipeInstructions, RecipeHistory};

#[component]
pub fn RecipeList() -> Element {
    let recipes_future = use_resource(|| async {
        content::get_all_recipes_server().await
    });

    let recipes_result = recipes_future.read();
    match &*recipes_result {
        Some(Ok(recipes)) => rsx! {
            div {
                id: "recipe-list",
                h1 { "Recipes" }
                ul {
                    for item in recipes.iter() {
                        li {
                            Link {
                                to: Route::RecipePost { slug: item.slug.to_string() },
                                "{item.title}"
                            }
                        }
                    }
                }
            }
        },
        Some(Err(e)) => rsx! { div { "Error: {e}" } },
        None => rsx! { div { "Loading..." } },
    }
}

#[component]
pub fn RecipePost(slug: String) -> Element {
    let recipe_future = use_resource(move || {
        let slug = slug.clone();
        async move {
            content::get_recipe_by_slug_server(slug).await
        }
    });

    let recipe_result = recipe_future.read();
    match &*recipe_result {
        Some(Ok(recipe)) => rsx! {
            div {
                id: "recipe-post",
                h1 { "{recipe.title}" }

                RecipeIngredients { ingredients: recipe.ingredients.clone() }
                RecipeInstructions { html: recipe.instructions_html.clone() }
                RecipeHistory { entries: recipe.history.clone() }

                Link {
                    to: Route::RecipeList {},
                    "← Back to recipes"
                }
            }
        },
        Some(Err(_)) => rsx! {
            div {
                h1 { "Recipe not found" }
                Link {
                    to: Route::RecipeList {},
                    "← Back to recipes"
                }
            }
        },
        None => rsx! { div { "Loading..." } },
    }
}
