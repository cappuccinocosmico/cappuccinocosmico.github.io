use dioxus::prelude::*;
use crate::{Route, BLOGS};

#[component]
pub fn BlogList() -> Element {
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

#[component]
pub fn BlogPost(slug: String) -> Element {
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
