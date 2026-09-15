use dioxus::prelude::*;
use crate::{Route, content};

#[component]
pub fn BlogList() -> Element {
    let blogs_future = use_resource(|| async {
        content::get_all_blogs_server().await
    });

    let blogs_result = blogs_future.read();
    match &*blogs_result {
        Some(Ok(blogs)) => rsx! {
            div {
                id: "blog-list",
                h1 { "Blogs" }
                ul {
                    for item in blogs.iter() {
                        li {
                            Link {
                                to: Route::BlogPost { slug: item.slug.to_string() },
                                "{item.title}"
                            }
                        }
                    }
                }
            }
        },
        Some(Err(e)) => rsx! { div { "Error: {e}" } },
        None => rsx! { div { "Loading..." } },
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    let blog_future = use_resource(move || {
        let slug = slug.clone();
        async move {
            content::get_blog_by_slug_server(slug).await
        }
    });

    let blog_result = blog_future.read();
    match &*blog_result {
        Some(Ok(item)) => rsx! {
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
        Some(Err(_)) => rsx! {
            div {
                h1 { "Blog not found" }
                Link {
                    to: Route::BlogList {},
                    "← Back to blogs"
                }
            }
        },
        None => rsx! { div { "Loading..." } },
    }
}
