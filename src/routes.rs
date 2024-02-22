use actix_web::{http::header::ContentType, web, HttpResponse, Responder};

use crate::api::{auth, post, user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(my_handler)));
    cfg.service(web::scope("/posts").configure(post::route::configure));
    cfg.service(web::scope("/auth").configure(auth::route::configure));
    cfg.service(web::scope("/users").configure(user::route::configure));
}

async fn my_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("my handler test!!!")
}
