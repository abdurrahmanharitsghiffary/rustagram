#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use actix_cors::Cors;
use actix_web::{http::header, rt::System, web, App, HttpServer};
mod app;
mod common;
mod entity;
use common::service::rabbitmq::{
    channel::create_rabbitmq_channel, consumer::email_consumer, queue_name::QueueName,
};
use env_logger::Env;
use lapin::ConnectionProperties;
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;
use tracing_actix_web::TracingLogger;

struct AppState {
    db: sqlx::Pool<Postgres>,
    ampq: deadpool::managed::Pool<deadpool_lapin::Manager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    let pg_pool_conn_max_size =
        env::var("PG_POOL_CONN_MAX_SIZE").unwrap_or_else(|_| "10".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = match PgPoolOptions::new()
        .max_connections(
            pg_pool_conn_max_size
                .parse()
                .expect("Must be a valid numeric string"),
        )
        .connect(&database_url)
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

    let ampq_pool_max_size = env::var("AMQP_POOL_MAX_SIZE").unwrap_or_else(|_| "10".to_string());
    let ampq_addr = env::var("AMQP_ADDR").expect("AMPQ_ADDR must be set");
    let manager = deadpool_lapin::Manager::new(ampq_addr, ConnectionProperties::default());
    let ampq_pool: deadpool_lapin::Pool = deadpool::managed::Pool::builder(manager)
        .max_size(
            ampq_pool_max_size
                .parse()
                .expect("Must be a valid numeric string"),
        )
        .build()
        .expect("Failed to create RabbitMQ pool");

    System::new().block_on(async {
        let channel = create_rabbitmq_channel(&[QueueName::EmailQueue], &ampq_pool).await;

        tokio::spawn(email_consumer(channel.clone()));

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
                    db: pg_pool.clone(),
                    ampq: ampq_pool.clone(),
                }))
                .service(
                    web::scope("/api/v1")
                        .configure(app::auth::controller::config)
                        .configure(app::user::controller::config),
                )
                .configure(app::health::controller::config)
                .wrap(cors)
                .wrap(TracingLogger::default())
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    })
}
