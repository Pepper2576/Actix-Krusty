use actix_web::{Responder, get};

#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello, world!"
}

#[get("/about")]
pub async fn about() -> impl Responder {
    "About page"
}

#[get("/health")]
pub async fn health() -> impl Responder {
    "OK"
}
