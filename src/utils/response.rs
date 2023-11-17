use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DieselError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "CustomError: {}", _0)]
    DieselError(DieselError),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DieselError(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
        }
    }
}
