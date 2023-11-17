use crate::models::user::CreateUserRequest;
use crate::models::user::NewUser;
use crate::services::user_services::UserService;
use crate::utils::response::CustomError;
use actix_web::error;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use std::sync::{Arc, RwLock};

#[get("/all")]
pub async fn get_all_users(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
) -> Result<HttpResponse, Error> {
    let user_service_clone = user_service.clone();
    let mut user_service_lock = user_service_clone
        .write()
        .map_err(|_| CustomError::DieselError(diesel::result::Error::NotFound))?;
    let users = user_service_lock
        .get_all_users()
        .map_err(|e| CustomError::DieselError(e))?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/find/{user_id}")]
pub async fn get_user_by_id(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let user_service_lock = match user_service.read() {
        Ok(user_service_lock) => user_service_lock,
        Err(_) => {
            return Err(error::ErrorInternalServerError(
                "Failed to acquire user service lock",
            ))
        }
    };

    if let Ok(Some(user)) = user_service_lock.get_user_by_id(user_id) {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[post("/create")]
pub async fn create_user(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
    new_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    let new_user = NewUser {
        username: &new_user.username,
        email: &new_user.email,
        password: new_user.password.as_deref(),
        tel: new_user.tel.as_deref(),
    };

    match user_service.write() {
        Ok(mut user_service_lock) => match user_service_lock.create_user(new_user) {
            Ok(user) => Ok(HttpResponse::Created().json(user)),
            Err(_) => Err(error::ErrorInternalServerError("Failed to create user")),
        },
        Err(_) => Err(error::ErrorInternalServerError(
            "Failed to acquire user service lock",
        )),
    }
}

#[put("/update/{user_id}")]
pub async fn update_user(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    match user_service.write() {
        Ok(mut user_service_lock) => {
            let user_id = user_id.into_inner();
            let updated_user = NewUser {
                username: &updated_user.username,
                email: &updated_user.email,
                password: updated_user.password.as_deref(),
                tel: updated_user.tel.as_deref(),
            };

            match user_service.update_user(user_id, &updated_user) {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_) => Err(error::ErrorInternalServerError("Failed to get user")),
            }
        }
        Err(_) => Err(error::ErrorInternalServerError(
            "Failed to acquire user service lock",
        )),
    }
}

#[delete("/delete/{user_id}")]
pub async fn delete_user(
    user_service: web::Data<UserService<'_>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    match user_service.delete_user(user_id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Failed to get user")),
    }
}
