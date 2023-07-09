use crate::controllers::user_controller;
use actix_web::{web, Scope};

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("/all", web::get().to(user_controller::get_all_users))
        .route(
            "/find/{user_id}",
            web::get().to(user_controller::get_user_by_id),
        )
        .route("/create", web::post().to(user_controller::create_user))
        .route(
            "/update/{user_id}",
            web::put().to(user_controller::update_user),
        )
        .route(
            "/delete/{user_id}",
            web::delete().to(user_controller::delete_user),
        )
}
