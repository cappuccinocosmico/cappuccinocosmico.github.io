use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::BlogList {},
                "Blogs"
            }
            Link {
                to: Route::RecipeList {},
                "Recipes"
            }
        }

        Outlet::<Route> {}
    }
}
