use std::env;

use super::authenticate_token::AuthenticationGuard;
use super::error::PostError;
use super::model::QueryCode;
use crate::api::login::model::{get_google_user, request_token, TokenClaims};
use crate::database::PgPool;
use actix_web::web::{Data, Query};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{info, warn};
use reqwest::header::LOCATION;
use uuid::Uuid;

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

    let email = google_user.email.to_lowercase();

    let user_id: String = google_user.id;

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let max_age = env::var("TOKEN_MAXAGE").expect("TOKEN_MAXAGE must be set");
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(max_age.parse::<i64>().unwrap())).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(
            60 * max_age.parse::<i64>().unwrap(),
            0,
        ))
        .http_only(true)
        .finish();

    let frontend_origin = env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
    let mut response = HttpResponse::Found();
    response.append_header((LOCATION, format!("{}{}", frontend_origin, state)));
    response.cookie(cookie);
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
