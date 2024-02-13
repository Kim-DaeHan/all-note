use super::authenticate_token::AuthenticationGuard;
use super::error::PostError;
use super::model::QueryCode;
use crate::api::login::model::{get_google_user, request_token};
use crate::database::PgPool;
use actix_web::Responder;
use actix_web::{
    web::{Data, Query},
    HttpResponse, Result,
};
use log::{info, warn};
use reqwest::header::LOCATION;

pub async fn google_oauth_handler(
    query: Query<QueryCode>,
    pool: Data<PgPool>,
) -> Result<impl Responder, PostError> {
    info!("로깅 테스트");
    warn!("로깅 테스트2");

    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        ));
    }

    let token_response = request_token(code.as_str()).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return Ok(HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message})));
    }

    let token_response = token_response.unwrap();
    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return Ok(HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message})));
    }

    let google_user = google_user.unwrap();

    println!("{:?}", google_user);

    let frontend_origin = "http://localhost:3000";
    let mut response = HttpResponse::Found();
    response.append_header((LOCATION, format!("{}{}", frontend_origin, state)));
    Ok(response.finish())
}

pub async fn get_me_handler(auth_guard: AuthenticationGuard, pool: Data<PgPool>) -> impl Responder {
    // let vec = data.db.lock().unwrap();

    // let user = vec
    //     .iter()
    //     .find(|user| user.id == Some(auth_guard.user_id.to_owned()));

    // let json_response = UserResponse {
    //     status: "success".to_string(),
    //     data: UserData {
    //         user: user_to_response(&user.unwrap()),
    //     },
    // };
    println!("{:?}", auth_guard);

    HttpResponse::Ok().json("adfasdf")
}
