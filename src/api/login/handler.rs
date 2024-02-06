use super::error::PostError;
use crate::database::PgPool;
use actix_web::Responder;
use actix_web::{http::header::ContentType, web, web::Data, HttpRequest, HttpResponse, Result};
use log::{error, info, warn};
use serde_json::to_vec;

pub async fn google_oauth_handler(pool: Data<PgPool>) -> Result<impl Responder, PostError> {
    info!("로깅 테스트");
    warn!("로깅 테스트2");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("google"))
}
