use super::error::UserError;
use super::model::{EmailQueryParam, UpdateUserData, User, UserData};
use crate::database::PgPool;
use actix_web::Responder;
use actix_web::{http::header::ContentType, web, web::Data, HttpRequest, HttpResponse, Result};
use log::{error, info, warn};
use serde_json::to_vec;

pub async fn get_users(pool: Data<PgPool>) -> Result<impl Responder, UserError> {
    info!("로깅 테스트");
    warn!("로깅 테스트2");

    let user_list = User::get_users_load(&pool).await;

    println!("{:?}", user_list);

    match User::get_users(&pool).await {
        Ok(user_data) => {
            let json_bytes = to_vec(&user_data).map_err(|err| {
                error!("Failed to serialize users to JSON: {:?}", err);
                UserError::InternalError
            })?;

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json_bytes))
        }
        Err(err) => {
            error!("Error get users data: {:?}", err);
            // 서버 에러
            Err(UserError::ValidationError {
                field: ("aaa").to_string(),
            })
        }
    }
}

pub async fn get_users_by_email(
    _body: web::Json<EmailQueryParam>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let email = &_body.email;

    match User::get_users_by_email(email, &pool).await {
        Ok(user_data) => {
            let json_bytes = to_vec(&user_data).map_err(|err| {
                error!("Failed to serialize users to JSON: {:?}", err);
                UserError::InternalError
            })?;

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json_bytes))
        }
        Err(err) => {
            error!("Error get users by id data: {:?}", err);
            Err(UserError::BadClientData)
        }
    }
}

pub async fn create_users(
    _body: web::Json<UserData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let user_data = _body.into_inner();

    match UserData::create_users(user_data, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("created new user")),
        Err(err) => {
            error!("Error created new user data: {:?}", err);
            Err(UserError::BadClientData)
        }
    }
}

pub async fn update_users(
    _body: web::Json<UpdateUserData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let user_data = _body.into_inner();

    match UpdateUserData::update_users(user_data, &pool).await {
        Ok(0) => {
            error!("Update failed");
            Err(UserError::BadClientData)
        }
        Err(err) => {
            error!("Error updated user data: {:?}", err);
            Err(UserError::InternalError)
        }
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("updated new user")),
    }
}

pub async fn delete_users_by_id(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    match req.match_info().get("id") {
        Some(user_id) => match User::delete_users_by_id(user_id, &pool).await {
            Ok(0) => {
                error!("Delete failed");
                Err(UserError::BadClientData)
            }
            Err(err) => {
                error!("Error deleted user data: {:?}", err);
                Err(UserError::InternalError)
            }
            Ok(_) => Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("deleted user data")),
        },
        None => {
            // user_id가 None인 경우의 동작
            error!("Error delete users data");
            Err(UserError::BadClientData)
        }
    }
}
