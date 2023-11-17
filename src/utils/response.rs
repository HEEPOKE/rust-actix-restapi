use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    DieselError(DieselError),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::DieselError(diesel_error) => {
                write!(f, "CustomError: {}", diesel_error)
            }
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DieselError(_) => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}
