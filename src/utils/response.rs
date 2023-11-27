use actix_web::{error::ResponseError, HttpResponse, http::{StatusCode, header::ContentType}};
use diesel::result::Error as DieselError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "CustomError: {}", _0)]
    DieselError(DieselError),

    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "not found")]
    NotFound,
}

#[derive(Debug, Display, Error)]
#[allow(dead_code)]
pub enum ResError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,
}

#[derive(Debug, Display, Error)]
#[allow(dead_code)]
enum ValidationErrorEnum {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DieselError(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
            CustomError::NotFound => HttpResponse::InternalServerError().body("not found"),
            CustomError::InternalError => HttpResponse::InternalServerError().body("Internal Server Error"),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::DieselError(_) | CustomError::NotFound => StatusCode::BAD_REQUEST,
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
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

impl ResponseError for ValidationErrorEnum {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ValidationErrorEnum::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}