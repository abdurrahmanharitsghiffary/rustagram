#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
mod app;
mod common;
mod config;
mod entity;
use common::service::rabbitmq::{
    channel::create_rabbitmq_channel, consumer::email_consumer, queue_name::QueueName,
};
use config::settings::AppSettings;
use env_logger::Env;
use lapin::ConnectionProperties;
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(info(description = "Rustagram Clone Open API"))]
struct ApiDoc;

struct AppState {
    settings: AppSettings,
    db: sqlx::Pool<Postgres>,
    ampq: deadpool::managed::Pool<deadpool_lapin::Manager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = AppSettings::new();

    if env::var_os("APP_RUST_LOG").is_none() {
        env::set_var("APP_RUST_LOG", "info");
    }

    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    let pg_pool = match PgPoolOptions::new()
        .max_connections(settings.database.pool_size)
        .connect(&settings.database_url)
        .await
    {
        Ok(pool) => {
            log::info!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            log::error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let manager =
        deadpool_lapin::Manager::new(&settings.rabbitmq_url, ConnectionProperties::default());
    let ampq_pool: deadpool_lapin::Pool = deadpool::managed::Pool::builder(manager)
        .max_size(settings.rabbitmq.pool_size)
        .build()
        .expect("Failed to create RabbitMQ pool");

    let channel = create_rabbitmq_channel(&[QueueName::EmailQueue], &ampq_pool).await;

    tokio::spawn(email_consumer(channel.clone(), settings.smtp.clone()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState {
                settings: settings.clone(),
                db: pg_pool.clone(),
                ampq: ampq_pool.clone(),
            }))
            .service(
                web::scope("/api/v1")
                    .configure(app::auth::controller::config)
                    .configure(app::user::controller::config),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(app::health::controller::config)
            .wrap(cors)
            .wrap(TracingLogger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
