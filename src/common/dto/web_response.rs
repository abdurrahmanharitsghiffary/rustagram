use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json;
use tracing;

#[derive(Serialize)]
pub struct WebResponse<T> {
    pub data: T,
    pub status_code: u16,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct PaginatedWebResponse<T> {
    pub data: T,
    pub status_code: u16,
    pub message: &'static str,
    pub metadata: Metadata,
}

#[derive(Serialize)]
pub struct WebErrorResponse {
    pub status_code: u16,
    pub message: &'static str,
    pub errors: Vec<WebErrorDetail>,
}

#[derive(Serialize)]
pub struct WebErrorDetail {
    pub message: &'static str,
    pub reason: &'static str,
    pub code: WebErrorCode,
}

#[derive(Serialize)]
pub enum WebErrorCode {
    NotFound,
}

#[derive(Serialize)]
pub struct Metadata {
    pub per_page: i32,
    pub page: i32,
    pub has_next: i32,
    pub has_previous: i32,
}

impl<T: Serialize> Responder for WebResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.serialize_and_respond()
    }
}

impl<T: Serialize> Responder for PaginatedWebResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.serialize_and_respond()
    }
}

impl Responder for WebErrorResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.serialize_and_respond()
    }
}

impl<T: Serialize> WebResponse<T> {
    fn serialize_and_respond(self) -> HttpResponse<BoxBody> {
        let body = match serde_json::to_string(&self) {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("Error serializing response body: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("Invalid status code: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        HttpResponse::build(status_code).body(body)
    }
}

impl<T: Serialize> PaginatedWebResponse<T> {
    fn serialize_and_respond(self) -> HttpResponse<BoxBody> {
        let body = match serde_json::to_string(&self) {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("Error serializing response body: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("Invalid status code: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        HttpResponse::build(status_code).body(body)
    }
}

impl WebErrorResponse {
    fn serialize_and_respond(self) -> HttpResponse<BoxBody> {
        let body = match serde_json::to_string(&self) {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("Error serializing response body: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("Invalid status codes: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        HttpResponse::build(status_code).body(body)
    }
}
