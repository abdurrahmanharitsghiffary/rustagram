use actix_web::{get, web, Either, Responder};

use crate::{
    common::dto::web_response::{WebErrorCode, WebErrorDetail, WebErrorResponse, WebResponse},
    AppState,
};

#[get("/ampq")]
async fn health_check_ampq(data: web::Data<AppState>) -> impl Responder {
    let conn = data.ampq.get().await;

    match conn {
        Ok(result) => Either::Left(WebResponse {
            status_code: 200,
            message: "Successfully connected to RabbitMQ".to_string(),
            data: (result.status().connected().to_string()),
        }),
        Err(err) => {
            log::error!("{}", err);
            Either::Right(WebErrorResponse {
                message: "Failed connecting to RabbitMQ".to_string(),
                status_code: 500,
                errors: vec![WebErrorDetail {
                    code: WebErrorCode::FailedConnect.value().to_string(),
                    message: err.to_string(),
                }],
            })
        }
    }
}

#[get("/db")]
async fn health_check_db(data: web::Data<AppState>) -> impl Responder {
    #[derive(sqlx::FromRow)]
    struct SampleDB {
        pub name: String,
    }

    let query_result = sqlx::query_as::<_, SampleDB>("SELECT * FROM users WHERE id = ? ")
        .bind("loler")
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(rows) => Either::Left(WebResponse {
            data: rows.name,
            message: "Success to query the database".to_string(),
            status_code: 200,
        }),
        Err(err) => {
            log::error!("{}", err);
            Either::Right(WebErrorResponse {
                message: "Failed to query the database.".to_string(),
                status_code: 500,
                errors: vec![WebErrorDetail {
                    code: WebErrorCode::FailedConnect.value().to_string(),
                    message: err.to_string(),
                }],
            })
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/health")
        .service(health_check_ampq)
        .service(health_check_db);

    conf.service(scope);
}
