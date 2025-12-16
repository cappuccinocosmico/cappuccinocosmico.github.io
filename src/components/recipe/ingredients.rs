use dioxus::prelude::*;

#[component]
pub fn RecipeIngredients(ingredients: &'static [&'static str]) -> Element {
    rsx! {
        div {
            class: "recipe-ingredients",
            h2 { "Ingredients" }
            ul {
                for ingredient in ingredients {
                    li { "{ingredient}" }
                }
            }
        }
    }
}
