#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use actix_web::{web, App, HttpServer};
mod app;
mod common;
mod config;
use app::user::controller as user_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .service(user_controller::get)
                .service(user_controller::update)
                .service(user_controller::delete)
                .service(user_controller::find)
                .service(user_controller::create),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
