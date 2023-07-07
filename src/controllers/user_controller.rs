use actix_web::{web, HttpResponse, Responder, error::Error};
use crate::services::UserService;
use crate::models::{NewUser, User};

pub async fn get_all_users(user_service: web::Data<UserService>) -> impl Responder {
    user_service.get_all_users()
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn create_user(
    user_service: web::Data<UserService>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    user_service.create_user(new_user.into_inner())
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn get_user_by_id(
    user_service: web::Data<UserService>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    user_service.get_user_by_id(user_id)
        .map(|user| {
            if let Some(user) = user {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::NotFound().finish()
            }
        })
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn update_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    user_service.update_user(user_id, updated_user.into_inner())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn delete_user(
    user_service: web::Data<UserService>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    user_service.delete_user(user_id)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|_| HttpResponse::InternalServerError().finish())
}
