use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/{id}")]
pub async fn get(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World {}", path))
}

#[get("/")]
pub async fn find() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[patch("/{id}")]
pub async fn update(path: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/{id}")]
pub async fn delete(path: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Created()
}
