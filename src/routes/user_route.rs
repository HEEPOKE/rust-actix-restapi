use actix_web::{web, Scope};

mod controller;

pub fn user_routes() -> Scope {
    web::scope("/apis")
        .route("/users/all", web::get().to(controllers::get_all_users))
        .route(
            "/users/find/{user_id}",
            web::get().to(controllers::get_user_by_id),
        )
        .route("/users/create", web::post().to(controllers::create_user))
        .route(
            "/users/update/{user_id}",
            web::put().to(controllers::update_user),
        )
        .route(
            "/users/delete/{user_id}",
            web::delete().to(controllers::delete_user),
        )
}
