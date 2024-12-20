use actix_web::{post, web, Responder};

use crate::common::dto::web_response::WebResponse;

use super::dto::ForgotPasswordRequestDTO;
use super::service::send_forgot_password_email;

#[post("/{id}")]
pub async fn request_forgot_password(body: web::Json<ForgotPasswordRequestDTO>) -> impl Responder {
    send_forgot_password_email(body.0);
    WebResponse {
        data: "NONON",
        message: "NONON",
        status_code: 200,
    }
}
