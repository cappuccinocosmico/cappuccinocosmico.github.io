use dioxus::prelude::*;
use crate::{Route, RECIPES};
use crate::components::recipe::{RecipeIngredients, RecipeInstructions, RecipeHistory};

#[component]
pub fn RecipeList() -> Element {
    rsx! {
        div {
            id: "recipe-list",
            h1 { "Recipes" }
            ul {
                for item in RECIPES.iter() {
                    li {
                        Link {
                            to: Route::RecipePost { slug: item.slug.to_string() },
                            "{item.title}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RecipePost(slug: String) -> Element {
    let recipe = RECIPES.iter().find(|r| r.slug == slug);

    match recipe {
        Some(recipe) => rsx! {
            div {
                id: "recipe-post",
                h1 { "{recipe.title}" }

                RecipeIngredients { ingredients: recipe.ingredients }
                RecipeInstructions { html: recipe.instructions_html }
                RecipeHistory { entries: recipe.history }

                Link {
                    to: Route::RecipeList {},
                    "← Back to recipes"
                }
            }
        },
        None => rsx! {
            div {
                h1 { "Recipe not found" }
                Link {
                    to: Route::RecipeList {},
                    "← Back to recipes"
                }
            }
        },
    }
}
