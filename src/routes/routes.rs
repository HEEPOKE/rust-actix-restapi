use actix_web::web;

use super::user_route;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user_route::user_routes());
}
