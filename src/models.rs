use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Bookmark {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub tag: String,
}

#[derive(Deserialize)]
pub struct TagFilter {
    pub tag: Option<String>,
}

#[derive(Deserialize)]
pub struct NewBookmark {
    pub title: String,
    pub url: String,
    pub tag: String,
}

#[derive(Deserialize)]
pub struct UpdateBookmark {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tag: Option<String>,
}
