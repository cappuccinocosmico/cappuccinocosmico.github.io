use dioxus::prelude::*;
use crate::models::Ingredient;
use num_rational::Rational64;

fn format_quantity(qty: Rational64) -> String {
    let numer = qty.numer();
    let denom = qty.denom();

    assert!(*denom > 0, "denominator must be positive");

    // If it's a whole number, just display the integer
    if denom == &1 {
        return numer.to_string();
    }

    // If numerator >= denominator, convert to mixed fraction
    if numer >= denom {
        let whole = numer / denom;
        let remainder = numer % denom;

        assert!(remainder >= 0, "remainder must be non-negative");

        if remainder == 0 {
            return whole.to_string();
        } else {
            return format!("{} {}/{}", whole, remainder, denom);
        }
    }

    // Otherwise, display as a simple fraction
    format!("{}/{}", numer, denom)
}

#[component]
pub fn RecipeIngredients(ingredients: Vec<Ingredient>) -> Element {
    rsx! {
        div {
            class: "recipe-ingredients",
            h2 { "Ingredients" }
            ul {
                for ingredient in ingredients {
                    li {
                        if let Some(qty) = ingredient.qty {
                            span { class: "qty", "{format_quantity(qty)}" }
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
