use crate::models::Bookmark;
use std::sync::Mutex;

pub struct AppState {
    pub bookmarks: Mutex<Vec<Bookmark>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            bookmarks: Mutex::new(vec![
                Bookmark {
                    id: 1,
                    title: "Example".to_string(),
                    url: "https://example.com".to_string(),
                    tag: "A".to_string(),
                },
                Bookmark {
                    id: 2,
                    title: "Another Example".to_string(),
                    url: "https://another.com".to_string(),
                    tag: "B".to_string(),
                },
            ]),
        }
    }
}
