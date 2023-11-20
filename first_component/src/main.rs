#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn main() {
    LaunchBuilder::new(App).launch();
}

pub fn App(cx: Scope) -> Element {
    render! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Axect".to_string(),
                score: 0,
                descendants: 0,
                time: Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}

//fn StoryListing(cx: Scope) -> Element {
//    let title = "title";
//    let by = "author";
//    let score = 0;
//    let time = chrono::Utc::now();
//    let comments = "comments";
//
//    render! {
//        div {
//            padding: "0.5rem",
//            position: "relative",
//            "{title} by {by} ({score}) {time} {comments}"
//        }
//    }
//}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

#[inline_props]
fn StoryListing(cx: Scope, story: StoryItem) -> Element {
    let StoryItem {
        id,
        title,
        by,
        score,
        time,
        kids,
        ..
    } = story;

    let comments = kids.len();

    render! {
        div {
            padding: "0.5rem",
            position: "relative",
            "{title} by {by} ({score}) {time} {comments}"
        }
    }
}
