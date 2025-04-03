use actix_web::{HttpResponse, ResponseError};
use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InternalServerError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::NotFound(ref message) => write!(f, "Not Found: {}", message),
            AppError::InternalServerError(ref message) => {
                write!(f, "Internal Server Error: {}", message)
            }
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::NotFound(_) => HttpResponse::NotFound().body(self.to_string()),
            AppError::InternalServerError(_) => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
        }
    }
}
