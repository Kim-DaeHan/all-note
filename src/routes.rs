use actix_web::{http::header::ContentType, web, HttpResponse, Responder};

use crate::api::post::route;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(my_handler)));
    // /posts 서브 라우터를 추가
    cfg.service(web::scope("/posts").configure(route::configure));
}

async fn my_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("my handler test!!!")
}
