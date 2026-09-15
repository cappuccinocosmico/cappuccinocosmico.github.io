use dioxus::prelude::*;

#[component]
pub fn RecipeInstructions(html: String) -> Element {
    rsx! {
        div {
            class: "recipe-instructions",
            h2 { "Instructions" }
            div {
                class: "prose",
                dangerous_inner_html: "{html}"
            }
        }
    }
}
