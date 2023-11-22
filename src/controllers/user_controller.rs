use crate::models::user::CreateUserRequest;
use crate::models::user::NewUser;
use crate::services::user_services::UserService;
use crate::utils::response::CustomError;
use actix_web::error;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use log::{error, info};
use std::sync::{Arc, RwLock};

#[utoipa::path(
    tag = "users",
    get,
    context_path = "/apis",
    path = "/users/all",
    responses(
        (status = 200, description = "get users successfully", body= [User]),
        (status = NOT_FOUND, description = "users was not found")
    ),
)]
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
    info!("Retrieved all users");
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    tag = "users",
    get,
    context_path = "/apis",
    path = "/users/find/{user_id}",
    responses(
        (status = 200, description = "find user with id success", body = User),
        (status = NOT_FOUND, description = "cannot find user with id")
    )
)]
#[get("/find/{user_id}")]
pub async fn get_user_by_id(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let mut user_service_lock = match user_service.write() {
        Ok(user_service_lock) => user_service_lock,
        Err(err) => {
            error!("find user with id error: {}", err);
            return Err(error::ErrorInternalServerError(
                "Failed to acquire user service lock",
            ));
        }
    };

    if let Ok(Some(user)) = user_service_lock.get_user_by_id(user_id) {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[utoipa::path(
    tag = "users",
    post,
    context_path = "/apis",
    path = "/users/create",
    request_body = NewUser,
    responses(
        (status = 200, description = "create user success", body = User),
        (status = NOT_FOUND, description = "cannot create user")
    )
)]
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
            Err(err) => {
                error!("create user error: {}", err);
                Err(error::ErrorInternalServerError("Failed to create user"))
            },
        },
        Err(err) => {
            error!("create user with error: {}", err);
            Err(error::ErrorInternalServerError(
                "Failed to acquire user service lock",
            ))
        }
    }
}

#[utoipa::path(
    tag = "users",
    put,
    context_path = "/apis",
    path = "/users/update/{user_id}",
    request_body = UpdateUser,
    responses(
        (status = 200, description = "update user success", body = User),
        (status = NOT_FOUND, description = "cannot update user")
    )
)]
#[put("/update/{user_id}")]
pub async fn update_user(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let updated_user = NewUser {
        username: &updated_user.username,
        email: &updated_user.email,
        password: updated_user.password.as_deref(),
        tel: updated_user.tel.as_deref(),
    };

    match user_service.write() {
        Ok(mut user_service_lock) => match user_service_lock.update_user(user_id, &updated_user) {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(err) => {
                error!("find user with id error: {}", err);
                Err(error::ErrorInternalServerError("Failed to get user"))
            }
        },
        Err(err) => {
            error!("update user with id error: {}", err);
            Err(error::ErrorInternalServerError(
                "Failed to acquire user service lock",
            ))
        }
    }
}

#[utoipa::path(
    tag = "users",
    delete,
    context_path = "/apis",
    path = "/users/delete/{user_id}",
    responses(
        (status = 200, description = "delete user success"),
        (status = NOT_FOUND, description = "cannot delete user")
    )
)]
#[delete("/delete/{user_id}")]
pub async fn delete_user(
    user_service: web::Data<Arc<RwLock<UserService<'_>>>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    user_service
        .write()
        .map_err(|_| error::ErrorInternalServerError("Failed to acquire user service lock"))?
        .delete_user(user_id)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|_| error::ErrorInternalServerError("Failed to delete user"))
}
