use dioxus::prelude::*;

mod components;
mod routes;

use components::Navbar;
use routes::{Home, BlogList, BlogPost, RecipeList, RecipePost};

// Include generated content from build.rs
include!(concat!(env!("OUT_DIR"), "/generated_content.rs"));

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blogs")]
    BlogList {},
    #[route("/blogs/:slug")]
    BlogPost { slug: String },
    #[route("/recipies")]
    RecipeList {},
    #[route("/recipies/:slug")]
    RecipePost { slug: String },
}

const FAVICON: Asset = asset!("assets/favicon.ico");
const MAIN_CSS: Asset = asset!("assets/main.css");
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
