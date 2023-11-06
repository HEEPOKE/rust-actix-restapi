use crate::models::user::NewUser;
use crate::models::user::CreateUserRequest; 
use crate::services::user_services::UserService;
use actix_web::{web, HttpResponse,Responder, Result, error::Error as ActixError};


pub async fn get_all_users(user_service: web::Data<UserService<'_>>) -> Result<impl HttpResponse, actix_web::Error> {
    match user_service.get_all_users() {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(_) => Err(actix_web::Error::from(HttpResponse::InternalServerError().finish())),
    }
}

pub async fn get_user_by_id(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    match user_service.get_user_by_id(user_id) {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Err(actix_web::Error::from(HttpResponse::InternalServerError().finish())),
    }
}

pub async fn create_user(
    user_service: web::Data<UserService<'_>>,
    new_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_user = NewUser {
        username: &new_user.username,
        email: &new_user.email,
        password: new_user.password.as_deref(),
        tel: new_user.tel.as_deref(),
    };

    match user_service.create_user(new_user) {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(_) => Err(actix_web::Error::from(HttpResponse::InternalServerError().finish())),
    }
}

pub async fn update_user(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let updated_user = NewUser {
        username: &updated_user.username,
        email: &updated_user.email,
        password: updated_user.password.as_deref(),
        tel: updated_user.tel.as_deref(),
    };

    match user_service.update_user(user_id, &updated_user) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(actix_web::Error::from(HttpResponse::InternalServerError().finish())),
    }
}

pub async fn delete_user(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    match user_service.delete_user(user_id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Err(actix_web::Error::from(HttpResponse::InternalServerError().finish())),
    }
}