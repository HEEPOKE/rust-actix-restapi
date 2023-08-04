use crate::models::user::{NewUser};
use crate::services::user_services::UserService;
use actix_web::{web, HttpResponse};

pub async fn get_all_users(user_service: web::Data<UserService<'_>>) -> Result<HttpResponse, HttpResponse> {
    user_service
        .get_all_users()
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|err| HttpResponse::InternalServerError().json(format!("Error: {:?}", err)))
}

pub async fn get_user_by_id(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, HttpResponse> {
    let user_id = user_id.into_inner();
    user_service
        .get_user_by_id(user_id)
        .map(|user| {
            if let Some(user) = user {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::NotFound().finish()
            }
        })
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn create_user<CreateUserRequest>(
    user_service: web::Data<UserService<'_>>,
    new_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let new_user = NewUser {
        username: &new_user.username,
        email: &new_user.email,
        password: new_user.password.as_deref(),
        tel: new_user.tel.as_deref(),
    };
    user_service
        .create_user(new_user)
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn update_user<UpdateUserRequest>(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let user_id = user_id.into_inner();
    let updated_user = NewUser {
        username: &updated_user.username,
        email: &updated_user.email,
        password: updated_user.password.as_deref(),
        tel: updated_user.tel.as_deref(),
    };
    user_service
        .update_user(user_id, &updated_user)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn delete_user(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, HttpResponse> {
    let user_id = user_id.into_inner();
    user_service
        .delete_user(user_id)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|_| HttpResponse::InternalServerError().finish())
}
