use super::error::PostError;
use super::model::{Post, PostData};
use crate::database::PgPool;
use actix_web::Responder;
use actix_web::{http::header::ContentType, web, web::Data, HttpRequest, HttpResponse, Result};
use log::{error, info, warn};
use serde_json::to_vec;

pub async fn get_posts(pool: Data<PgPool>) -> Result<impl Responder, PostError> {
    info!("로깅 테스트");
    warn!("로깅 테스트2");

    let post_list = Post::get_posts_load(&pool).await;

    println!("{:?}", post_list);

    match Post::get_posts(&pool).await {
        Ok(post_data) => {
            let json_bytes = to_vec(&post_data).map_err(|err| {
                error!("Failed to serialize posts to JSON: {:?}", err);
                PostError::InternalError
            })?;

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json_bytes))
        }
        Err(err) => {
            error!("Error get posts data: {:?}", err);
            // 서버 에러
            Err(PostError::ValidationError {
                field: ("aaa").to_string(),
            })
        }
    }
}

pub async fn get_posts_by_id(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    match req.match_info().get("id") {
        Some(post_id) => match Post::get_posts_by_id(post_id, &pool).await {
            Ok(post_data) => {
                let json_bytes = to_vec(&post_data).map_err(|err| {
                    error!("Failed to serialize posts to JSON: {:?}", err);
                    PostError::InternalError
                })?;

                Ok(HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(json_bytes))
            }
            Err(err) => {
                error!("Error get posts by id data: {:?}", err);
                Err(PostError::BadClientData)
            }
        },
        None => {
            // post_id가 None인 경우의 동작
            error!("Error get posts data");
            Err(PostError::BadClientData)
        }
    }
}

pub async fn create_posts(
    _body: web::Json<PostData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let post_data = _body.into_inner();

    match PostData::create_posts(post_data, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("created new post")),
        Err(err) => {
            error!("Error created new post data: {:?}", err);
            Err(PostError::BadClientData)
        }
    }
}

pub async fn update_posts(
    _body: web::Json<PostData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let post_data = _body.into_inner();
    match post_data.id {
        Some(_) => match PostData::update_posts(post_data, &pool).await {
            Ok(0) => {
                error!("Update failed");
                Err(PostError::BadClientData)
            }
            Err(err) => {
                error!("Error updated post data: {:?}", err);
                Err(PostError::InternalError)
            }
            Ok(_) => Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("updated new post")),
        },
        None => {
            // post_data.id가 None인 경우의 동작
            Err(PostError::BadClientData)
        }
    }
}

pub async fn delete_posts_by_id(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    match req.match_info().get("id") {
        Some(post_id) => match Post::delete_posts_by_id(post_id, &pool).await {
            Ok(0) => {
                error!("Delete failed");
                Err(PostError::BadClientData)
            }
            Err(err) => {
                error!("Error deleted post data: {:?}", err);
                Err(PostError::InternalError)
            }
            Ok(_) => Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("deleted post data")),
        },
        None => {
            // post_id가 None인 경우의 동작
            error!("Error delete posts data");
            Err(PostError::BadClientData)
        }
    }
}
