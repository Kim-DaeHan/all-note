use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum LoginError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            code: self.status_code().as_u16(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            LoginError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::BadClientData => StatusCode::BAD_REQUEST,
            LoginError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            LoginError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
