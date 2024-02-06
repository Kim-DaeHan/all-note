use crate::api::login::handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/google").route(web::get().to(handler::google_oauth_handler)));
}
