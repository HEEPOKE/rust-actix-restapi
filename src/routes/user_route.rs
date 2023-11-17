use crate::controllers::user_controller;
use actix_web::{web, Scope};

pub fn user_routes() -> Scope {
    web::scope("/users")
        .service(user_controller::get_all_users)
        .service(user_controller::get_user_by_id)
        .service(user_controller::create_user)
        .service(user_controller::update_user)
        .service(user_controller::delete_user)
}
