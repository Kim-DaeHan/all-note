use crate::api::auth::handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/google").route(web::get().to(handler::google_oauth_handler)));
    cfg.service(web::resource("/users").route(web::get().to(handler::get_me_handler)));
    cfg.service(web::resource("/logout").route(web::get().to(handler::logout_handler)));
}
