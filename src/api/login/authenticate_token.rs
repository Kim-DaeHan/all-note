use std::{
    env,
    future::{ready, Ready},
};

use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::json;

use crate::database::PgPool;

use super::model::TokenClaims;

#[derive(Debug)]
pub struct AuthenticationGuard {
    pub user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "You are not logged in, please provide token"}),
            )));
        }

        let data = req.app_data::<web::Data<PgPool>>().unwrap();

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let decode = decode::<TokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => {
                println!("data: {:?}", data);
                println!("token.claims.sub: {:?}", Some(token.claims.sub.to_owned()));

                ready(Ok(AuthenticationGuard {
                    user_id: token.claims.sub,
                }))
            }
            Err(_) => ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token or usre doesn't exists"}),
            ))),
        }
    }
}
