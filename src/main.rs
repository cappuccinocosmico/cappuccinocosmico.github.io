use dioxus::prelude::*;

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
const HEADER_SVG: Asset = asset!("assets/header.svg");
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

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog list page
#[component]
fn BlogList() -> Element {
    rsx! {
        div {
            id: "blog-list",
            h1 { "Blogs" }
            ul {
                for item in BLOGS.iter() {
                    li {
                        Link {
                            to: Route::BlogPost { slug: item.slug.to_string() },
                            "{item.title}"
                        }
                    }
                }
            }
        }
    }
}

/// Individual blog post page
#[component]
fn BlogPost(slug: String) -> Element {
    let item = BLOGS.iter().find(|item| item.slug == slug);

    match item {
        Some(item) => rsx! {
            div {
                id: "blog-post",
                h1 { "{item.title}" }
                div {
                    class: "prose prose-lg",
                    dangerous_inner_html: "{item.html}"
                }
                Link {
                    to: Route::BlogList {},
                    "← Back to blogs"
                }
            }
        },
        None => rsx! {
            div {
                h1 { "Blog not found" }
                Link {
                    to: Route::BlogList {},
                    "← Back to blogs"
                }
            }
        },
    }
}

/// Recipe list page
#[component]
fn RecipeList() -> Element {
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

/// Individual recipe post page
#[component]
fn RecipePost(slug: String) -> Element {
    let item = RECIPES.iter().find(|item| item.slug == slug);

    match item {
        Some(item) => rsx! {
            div {
                id: "recipe-post",
                h1 { "{item.title}" }
                div {
                    class: "prose prose-lg",
                    dangerous_inner_html: "{item.html}"
                }
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

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
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
