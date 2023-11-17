use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use diesel::result::Error as DieselError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "CustomError: {}", _0)]
    DieselError(DieselError),
}

#[derive(Debug, Display, Error)]
#[allow(dead_code)]
pub enum ResError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DieselError(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::DieselError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for ResError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ResError::InternalError => HttpResponse::InternalServerError().body("Internal Server Error"),
            ResError::BadClientData => HttpResponse::BadRequest().body("Bad Request"),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ResError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ResError::BadClientData => StatusCode::BAD_REQUEST,
        }
    }
}