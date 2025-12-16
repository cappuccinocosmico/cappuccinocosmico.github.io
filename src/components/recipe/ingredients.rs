use dioxus::prelude::*;
use crate::Ingredient;

#[component]
pub fn RecipeIngredients(ingredients: &'static [Ingredient]) -> Element {
    rsx! {
        div {
            class: "recipe-ingredients",
            h2 { "Ingredients" }
            ul {
                for ingredient in ingredients {
                    li {
                        if let Some(qty) = ingredient.qty {
                            span { class: "qty", "{qty}" }
                            " "
                        }
                        if !ingredient.unit.is_empty() {
                            span { class: "unit", "{ingredient.unit}" }
                            " "
                        }
                        span { class: "name", "{ingredient.name}" }
                        if !ingredient.note.is_empty() {
                            " "
                            span { class: "note", "({ingredient.note})" }
                        }
                    }
                }
            }
        }
    }
}
