use actix_web::{web, Scope};

mod controllers;

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("", web::get().to(controllers::get_all_users))
        .route("", web::post().to(controllers::create_user))
        .route("/{user_id}", web::get().to(controllers::get_user_by_id))
        .route("/{user_id}", web::put().to(controllers::update_user))
        .route("/{user_id}", web::delete().to(controllers::delete_user))
}
