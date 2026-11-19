use crate::models::{Bookmark, NewBookmark, TagFilter, UpdateBookmark};
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, delete, get, patch, post, put, web};

#[get("/bookmarks")]
pub async fn list_bookmarks(
    data: web::Data<AppState>,
    query: web::Query<TagFilter>,
) -> impl Responder {
    let bookmarks = data.bookmarks.lock().unwrap();
    let filter: Vec<Bookmark> = bookmarks
        .iter()
        .filter(|b| match &query.tag {
            Some(tag) => &b.tag == tag,
            None => true,
        })
        .cloned()
        .collect();
    web::Json(filter)
}
#[get("/bookmarks/{id}")]
pub async fn get_bookmark(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let bookmarks: std::sync::MutexGuard<'_, Vec<Bookmark>> = data.bookmarks.lock().unwrap();
    match bookmarks.iter().find(|b: &&Bookmark| b.id == id) {
        Some(bookmark) => HttpResponse::Ok().json(bookmark),
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}

#[post("/bookmarks")]
pub async fn create_bookmark(
    data: web::Data<AppState>,
    new_bookmark: web::Json<crate::models::NewBookmark>,
) -> impl Responder {
    let mut bookmarks = data.bookmarks.lock().unwrap();
    let id = bookmarks.last().map_or(1, |b| b.id + 1);
    let bookmark = Bookmark {
        id,
        title: new_bookmark.title.clone(),
        url: new_bookmark.url.clone(),
        tag: new_bookmark.tag.clone(),
    };
    bookmarks.push(bookmark.clone());
    HttpResponse::Created().json(bookmark)
}

#[put("/bookmarks/{id}")]
pub async fn update_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    updated_bookmark: web::Json<NewBookmark>,
) -> impl Responder {
    let id = path.into_inner();
    let changes = updated_bookmark.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();
    match bookmarks.iter_mut().find(|b| b.id == id) {
        Some(bookmark) => {
            bookmark.title = changes.title;
            bookmark.url = changes.url;
            bookmark.tag = changes.tag;
            HttpResponse::Ok().json(bookmark)
        }
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}

#[patch("/bookmarks/{id}")]
pub async fn patch_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    updated_bookmark: web::Json<UpdateBookmark>,
) -> impl Responder {
    let id = path.into_inner();
    let changes = updated_bookmark.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();
    match bookmarks.iter_mut().find(|b| b.id == id) {
        Some(bookmark) => {
            if let Some(title) = changes.title {
                bookmark.title = title;
            }
            if let Some(url) = changes.url {
                bookmark.url = url;
            }
            if let Some(tag) = changes.tag {
                bookmark.tag = tag;
            }
            HttpResponse::Ok().json(bookmark)
        }
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}

#[delete("/bookmarks/{id}")]
pub async fn delete_bookmark(data: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();
    match bookmarks.iter().position(|b| b.id == id) {
        Some(pos) => {
            bookmarks.remove(pos);
            HttpResponse::Ok().body("Bookmark deleted")
        }
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}
