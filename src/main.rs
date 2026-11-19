mod handlers;
mod models;
mod state;

use crate::handlers::{bookmarks, general};
use crate::state::AppState;
use actix_web::{App, HttpServer, web};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());

    println!(
        "Starting server on port {}",
        env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    );

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(general::hello)
            .service(general::about)
            .service(general::health)
            .service(bookmarks::list_bookmarks)
            .service(bookmarks::get_bookmark)
            .service(bookmarks::create_bookmark)
            .service(bookmarks::update_bookmark)
            .service(bookmarks::patch_bookmark)
            .service(bookmarks::delete_bookmark)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;
    Ok(())
}
