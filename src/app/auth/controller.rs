use actix_web::{
    get, post,
    web::{self, Either, Form, Json},
    Responder,
};
use validator::Validate;

use crate::common::dto::web_response::{
    WebErrorCode, WebErrorDetail, WebErrorResponse, WebResponse,
};

use super::dto::{ForgotPasswordRequestDTO, LoginDTO, RegisterDTO, ResetPasswordDTO};
use super::service as auth_service;

#[post("/register")]
async fn request_forgot_password(
    body: Either<Json<ForgotPasswordRequestDTO>, Form<ForgotPasswordRequestDTO>>,
) -> impl Responder {
    let ForgotPasswordRequestDTO { email } = body.into_inner();
    auth_service::send_forgot_password_email(email).await;
    WebResponse {
        data: {},
        message: "A password reset email will be sent shortly if the email address is registered."
            .to_string(),
        status_code: 200,
    }
}

#[post("/reset-password")]
async fn reset_password(
    body: Either<Json<ResetPasswordDTO>, Form<ResetPasswordDTO>>,
) -> impl Responder {
    let inner_body = body.into_inner();

    match inner_body.validate() {
        Ok(_) => Either::Left(WebResponse {
            data: {},
            message: "Reset Password Success".to_string(),
            status_code: 200,
        }),
        Err(err) => {
            let validation_errors =
                err.errors()
                    .into_iter()
                    .map(|validation_error| WebErrorDetail {
                        code: WebErrorCode::NotFound.value().to_string(),
                        message: validation_error.0.to_string(),
                    });

            Either::Right(WebErrorResponse {
                errors: validation_errors.collect(),
                message: "Error".to_string(),
                status_code: 422,
            })
        }
    }
}

#[post("/login")]
async fn login(body: Either<Json<LoginDTO>, Form<LoginDTO>>) -> impl Responder {
    auth_service::login(body.into_inner()).await;
    WebResponse {
        data: {},
        message: "Login success".to_string(),
        status_code: 200,
    }
}

#[post("/register")]
async fn register(body: Either<Json<RegisterDTO>, Form<RegisterDTO>>) -> impl Responder {
    auth_service::register_account(body.into_inner()).await;
    WebResponse {
        data: {},
        message: "Sucessfully register account".to_string(),
        status_code: 200,
    }
}

#[post("/revoke")]
async fn revoke_refresh_token() -> impl Responder {
    auth_service::revoke_token(String::from("Some Token")).await;
    WebResponse {
        data: {},
        message: "Sucessfully revoke refresh token".to_string(),
        status_code: 200,
    }
}

#[get("/me")]
async fn get_user_details() -> impl Responder {
    WebResponse {
        data: {},
        message: "My name is Jamal Boolean".to_string(),
        status_code: 200,
    }
}

#[post("/verify-account")]
async fn verify_account() -> impl Responder {
    WebResponse {
        data: {},
        message: "Verify your account".to_string(),
        status_code: 200,
    }
}

#[post("/request-verify-account")]
async fn request_verify_account() -> impl Responder {
    WebResponse {
        data: {},
        message: "Verify your account".to_string(),
        status_code: 200,
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(login)
        .service(request_forgot_password)
        .service(reset_password)
        .service(register)
        .service(revoke_refresh_token)
        .service(get_user_details)
        .service(verify_account)
        .service(request_verify_account);

    conf.service(scope);
}
