use crate::models::user::NewUser;
use crate::models::user::CreateUserRequest; 
use crate::services::user_services::UserService;
use actix_web::{web, HttpResponse, Result, error::Error as ActixError};

pub async fn get_all_users(user_service: web::Data<UserService<'_>>) -> Result<HttpResponse, ActixError> {
    let users = user_service.get_all_users()?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    let user_id = user_id.into_inner();
    let user = user_service.get_user_by_id(user_id)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn create_user(
    user_service: web::Data<UserService<'_>>,
    new_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, ActixError> {
    let new_user = NewUser {
        username: &new_user.username,
        email: &new_user.email,
        password: new_user.password.as_deref(),
        tel: new_user.tel.as_deref(),
    };

    let user = user_service.create_user(new_user)?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn update_user(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, ActixError> {
    let user_id = user_id.into_inner();
    let updated_user = NewUser {
        username: &updated_user.username,
        email: &updated_user.email,
        password: updated_user.password.as_deref(),
        tel: updated_user.tel.as_deref(),
    };
    let user = user_service.update_user(user_id, &updated_user)?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    let user_id = user_id.into_inner();
    user_service.delete_user(user_id)?;
    Ok(HttpResponse::NoContent().finish())
}
