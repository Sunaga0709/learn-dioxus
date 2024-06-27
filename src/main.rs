#![allow(non_snake_case)]
#![allow(unused)]

use chrono::{DateTime, Local, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::Level;

// #[derive(Clone, Routable, Debug, PartialEq)]
// enum Route {
//     #[route("/")]
//     Home {},
//     #[route("/blog/:id")]
//     Blog { id: i32 },
//     #[route("/example")]
//     Example {},
//     #[route("/stories")]
//     StoryListing { story: ReadOnlySignal<StoryItem> },
// }

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        // Router::<Route> {}
        StoryListing {
            story: StoryItem {
                id: 0,
                title: String::from("hello hackernew"),
                url: None,
                text: None,
                by: String::from("Sunaga"),
                score: 0,
                descendants: 0,
                time: Utc::now(),
                kids: vec![],
                r#type: String::new(),
            }
        }
    }
}

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

// #[component]
// fn Home() -> Element {
//     let mut count = use_signal(|| 0);

//     rsx! {
//         Link {
//             to: Route::Blog {
//                 id: count()
//             },
//             "Go to blog"
//         }
//         div {
//             h1 { "High-Five counter: {count}" }
//             button { onclick: move |_| count += 1, "Up high!" }
//             button { onclick: move |_| count -= 1, "Down low!" }
//         }
//     }
// }

// #[component]
// fn Example() -> Element {
//     let count = use_signal(|| 0_i32);

//     let title = " Title ";
//     let by = " Sunaga ";
//     let score = 0;
//     let time = chrono::Utc::now();
//     let comments = " comments ";

//     rsx! {
//         Link {
//             to: Route::Home {},
//             "Go to home",
//         },
//         br {},
//         Link {
//             to: Route::Blog{id: count()},
//             "Go to blog",
//         },
//         div{
//             padding: "9.5rem",
//             position: "relative",
//             "{title} by {by} ({score}) {time} {comments}"
//         }

//     }
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct StoryPageData {
    #[serde(flatten)]
    item: StoryItem,
    #[serde(default)]
    comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct StoryItem {
    id: i64,
    title: String,
    url: Option<String>,
    text: Option<String>,
    #[serde(default)]
    by: String,
    #[serde(default)]
    score: i64,
    #[serde(default)]
    descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    kids: Vec<i64>,
    r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Comment {
    id: i64,
    #[serde(default)]
    by: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    kids: Vec<i64>,
    #[serde(default)]
    sub_comments: Vec<Comment>,
    r#type: String,
}

#[component]
fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if *score == 1 { "point" } else { "points" });
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            "comment"
        } else {
            "comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            div {
                font_size: "1.5rem",
                a {
                    href: url, "{title}"
                },
                a{
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})",
                }
            },
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    "{score}"
                },
                div {
                    padding_left: "0.5rem",
                    "by {by}",
                },
                div {
                    padding_left: "0.5rem",
                    "{time}",
                },
                div {
                    padding_left: "0.5rem",
                    "{comments}",
                },
            }
        }
    }
}
