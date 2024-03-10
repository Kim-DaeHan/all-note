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

// https://accounts.google.com/signin/oauth/id?authuser=1&part=AJi8hAOV3bDJfjMiWxZmMBX3UHRjlEKNLVITKzcfmxdm-Y3p_IXWszqDypzr2xGinr-iPDy2jTvPkneDxJK-se_YokHAbtSvgTAp0qOH6NSJLvNcwCi0suxEx7I1Z8EDuq6QKPVWG5W-2tnvAqAW6KhWYFoGEa1stt9Ded6vLvMNUdw8TcxmwVVtH5JItRWFrM2xgA1SrqpNm82-mh9cCdnc3kMz7nFXUGjkAqyskn8NQfhT20QJriCQAQXPkQSoTXcXRxKMwMT1pCuSIsEZXUFLDDPiPY5fue7ka2xXAsk6U765SJXLm8BFwTls8uUJJP799ciTGdmuAA0bekTRkRbQLVrIV_sw2mGgUqkDUKGtNjdYj3YcZE-yNvOOmH8KuqaPgoM9VtoQWQs2hWz7QK98V66Jh2_kJtY_J-KTuO2RcEjj0mggLH03CJ8BB616Fua8cFHXIWMULO9ri3y4gSGPikHAmWUwEX0nxMaNOkZ977lrvbb_GBZsC04QAINrUHZBd1-p0Oqp9EA9R_gfxlTNAujFRdoQ20n5vFxw5bDiz-z1jZVUvx-HePALm28vnU1qOxOX4GUgDlbe5uCrAKtyPkWuu8V6ZgMHSESx4-UQ8LzWUaSQjsu6q_S5QBnFx5HJPh9mQNVe0lU83ddhXnbH26a1GP3vdOavFRm4vK6m83xsP1UQEowJk1nNBIh1O-sRzdX_5c2qFiYpZ4mbIMw2ZXS7VjBlo_jBtMRoTmqfOoKEANsoa1WRQCkjTr8RVH1m9iwfhwdG5QN1KbtOTJozWHaSSY31UhyWsbHppHtx8jMC0tvtOqqb2vke7BdDoUEOIsjAXhaeggFoEkDr-_UR82a3X5eWVPORIBJrwtog9I8nZzQivn03jCAqeEnFk25OQYaaGh7nss0x1ZSELJXPytt4vIlusdMlmCkFAquVAfqaYicQFa9I5uMbTuAfaEvDKq3l4pCAXPJ-P9mhvT9A21izkiyxTvnvpvCfsHjuHdqs0B4w_GdE3l12tPsK-8HJaArgdH1jivpLhiZaAJ2SRH78R3e0fcnwEn-lAR3g1Oo9tFO9iYEMt51eu0H6fn8-8QipWZOrhJmNGsntF9nKPG6rm84z_bk00y-XKGLN9b7cvrBza3M&flowName=GeneralOAuthFlow&as=S2098109018%3A1709439746845592&client_id=296653457840-1ckfc04i9jsfgc20sm8bglfkbg592alg.apps.googleusercontent.com&rapt=AEjHL4PVehYZNktenBMyKZIGp_qV5pdgX6MzwRkgpv2-6NJ8q_S5Zl1UbLbkFO7SstAWLxLeg-OoxMLhAgVDLOUaT0RZ3RGQ9w&theme=glif#
