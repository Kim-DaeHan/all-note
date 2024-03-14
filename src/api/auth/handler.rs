use std::env;

use super::authenticate_token::AuthenticationGuard;
use super::error::LoginError;
use super::model::QueryCode;
use crate::api::auth::model::{get_google_user, request_token, TokenClaims};
use crate::api::user::model::{UpdateUserData, User, UserData};
use crate::database::PgPool;
use actix_web::web::{Data, Query};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info, warn};
use reqwest::header::LOCATION;

pub async fn google_oauth_handler(
    query: Query<QueryCode>,
    pool: Data<PgPool>,
) -> Result<impl Responder, LoginError> {
    info!("로깅 테스트");
    warn!("로깅 테스트2");

    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        ));
    }
    println!("code: {:?}", query);

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

    // user email로 find
    let user = User::get_users_by_email(&email, &pool).await;

    let user_id: String;

    // if 문으로 유저가 존재하면 update, 없으면 insert
    if user.is_ok() {
        let user = user.unwrap();
        user_id = user.id;

        let user_data = UpdateUserData {
            id: user_id.clone(),
            google_id: google_user.id,
            email: email.to_owned(),
            user_name: google_user.name,
            verified: google_user.verified_email,
            provider: "Google".to_string(),
            photo: google_user.picture,
            updated_at: None,
        };

        let update_res = UpdateUserData::update_users(user_data, &pool).await;

        if let Err(err) = update_res {
            error!("Error updating user data: {:?}", err);
            return Err(LoginError::InternalError);
        }

        if let Ok(0) = update_res {
            error!("Update failed");
            return Err(LoginError::BadClientData);
        }
    } else {
        let user_data = UserData {
            id: None,
            google_id: google_user.id,
            email: email.to_owned(),
            user_name: google_user.name,
            verified: google_user.verified_email,
            provider: "Google".to_string(),
            photo: google_user.picture,
        };

        let create_res = UserData::create_users(user_data, &pool).await;

        if let Err(err) = create_res {
            error!("Error created new user data: {:?}", err);
            return Err(LoginError::BadClientData);
        }

        user_id = create_res.unwrap();
    }

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

pub async fn get_me_handler(
    auth_guard: AuthenticationGuard,
    pool: Data<PgPool>,
) -> Result<impl Responder, LoginError> {
    let user_id = auth_guard.user_id;

    match User::get_users_by_id(&user_id, &pool).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            error!("Error getting user by ID: {:?}", err);
            Err(LoginError::BadClientData)
        }
    }
}

pub async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}
