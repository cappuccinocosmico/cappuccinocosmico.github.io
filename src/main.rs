use dioxus::prelude::*;

#[cfg(feature = "server")]
use server_fn::{ServerFnError, server};

mod components;
mod content;
mod models;
mod routes;

use components::Navbar;
use routes::{Home, BlogList, BlogPost, RecipeList, RecipePost};

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

// Server function for SSG route discovery
#[cfg(feature = "server")]
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // Start with non-dynamic routes from the Route enum
    let mut routes = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    // Add all recipe routes
    let recipes = content::get_recipes();
    for recipe in recipes {
        routes.push(format!("/recipies/{}", recipe.slug));
    }

    // Add all blog routes
    let blogs = content::get_blogs();
    for blog in blogs {
        routes.push(format!("/blogs/{}", blog.slug));
    }

    Ok(routes)
}

fn main() {
    LaunchBuilder::new().launch(App);
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
