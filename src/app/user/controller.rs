use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/{id}")]
async fn find(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World {}", path))
}

#[get("/")]
async fn find_many() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[patch("/{id}")]
async fn update(path: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[delete("/{id}")]
async fn delete(path: web::Path<String>) -> impl Responder {
    HttpResponse::NoContent()
}

#[post("/")]
async fn create() -> impl Responder {
    HttpResponse::Created()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(find)
        .service(find_many)
        .service(update)
        .service(delete)
        .service(create);

    conf.service(scope);
}
