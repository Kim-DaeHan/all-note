use crate::api::post::handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(handler::get_posts))
            .route(web::post().to(handler::create_posts))
            .route(web::put().to(handler::update_posts)),
    );

    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(handler::get_posts_by_id))
            .route(web::delete().to(handler::delete_posts_by_id)),
    );
}
