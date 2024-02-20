use crate::api::user::handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(handler::get_users))
            .route(web::post().to(handler::create_users))
            .route(web::put().to(handler::update_users)),
    );

    cfg.service(web::resource("/email").route(web::post().to(handler::get_users_by_email)));

    cfg.service(web::resource("/{id}").route(web::delete().to(handler::delete_users_by_id)));
}
