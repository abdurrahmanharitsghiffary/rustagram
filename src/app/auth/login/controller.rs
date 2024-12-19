use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/{id}")]
pub async fn login(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World {}", path))
}
