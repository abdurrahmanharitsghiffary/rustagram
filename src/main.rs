#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use actix_cors::Cors;
use actix_web::{get, http::header, web, App, Either, HttpServer, Responder};
mod app;
mod common;
mod config;
use common::dto::web_response::{WebErrorCode, WebErrorDetail, WebErrorResponse, WebResponse};
use env_logger::Env;
use lapin::ConnectionProperties;
use sqlx::{postgres::PgPoolOptions, Postgres};
use tracing_actix_web::TracingLogger;

struct AppState {
    db: sqlx::Pool<Postgres>,
    ampq: deadpool::managed::Pool<deadpool_lapin::Manager>,
}

#[get("/health/ampq")]
async fn health_check_ampq(app_state: web::Data<AppState>) -> impl Responder {
    let conn = app_state.ampq.get().await;

    match conn {
        Ok(result) => Either::Left(WebResponse {
            status_code: 200,
            message: "Successfully connected to RabbitMQ".to_string(),
            data: (result.status().connected().to_string()),
        }),
        Err(err) => Either::Right(WebErrorResponse {
            message: "Failed connecting to RabbitMQ".to_string(),
            status_code: 500,
            errors: vec![WebErrorDetail {
                code: WebErrorCode::FailedConnect.value().to_string(),
                message: err.to_string(),
            }],
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    let pg_pool_conn_max_size =
        std::env::var("PG_POOL_CONN_MAX_SIZE").unwrap_or_else(|_| 10.to_string());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = match PgPoolOptions::new()
        .max_connections(
            pg_pool_conn_max_size
                .parse()
                .expect("Must be a valid number string"),
        )
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let ampq_pool_max_size = std::env::var("AMQP_POOL_MAX_SIZE").unwrap_or_else(|_| 10.to_string());
    let ampq_addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "AMQP_ADDR=amqp://guest:guest@127.0.0.1:5672".into());
    let manager = deadpool_lapin::Manager::new(ampq_addr, ConnectionProperties::default());
    let ampq_pool: deadpool_lapin::Pool = deadpool::managed::Pool::builder(manager)
        .max_size(
            ampq_pool_max_size
                .parse()
                .expect("Must be a valid number string"),
        )
        .build()
        .expect("can create pool");

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
            .service(health_check_ampq)
            .wrap(cors)
            .wrap(TracingLogger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
