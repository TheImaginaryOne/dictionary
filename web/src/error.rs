use database::search::SearchError;
use database::diesel::result::Error as DieselError;
use std::error::Error;
use actix_web::{ResponseError, HttpResponse};
use std::fmt::{Formatter, Display, self};
use serde::Serialize;

#[derive(Debug)]
pub enum DictError {
    Search(SearchError),
    Database(DieselError),
    Actix,
}
impl Error for DictError {}

/// Converts this error into an http response. This method is called
/// when this DictError is returned from a handler.
impl ResponseError for DictError {
    fn error_response(&self) -> HttpResponse {
        #[derive(Serialize)]
        struct R {
            message: String,
        }
        impl R {
            fn new(message: String) -> Self {
                Self { message }
            }
        }
        // return HttpResponse
        match self {
            DictError::Search(_) => {
                HttpResponse::BadRequest().json(R::new(format!("{}", self)))
            },
            DictError::Database(_) => {
                HttpResponse::InternalServerError().json(R::new(format!("{}", self)))
            }
            DictError::Actix => {
                HttpResponse::InternalServerError().json(R::new(format!("{}", self)))
            }
        }
    }
}
/// Required for implementing std::error::Error
impl Display for DictError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            DictError::Search(e) => {
                write!(fmt, "Search error: {}", e)
            },
            DictError::Database(e) => write!(fmt, "Database error: {}", e),
            DictError::Actix => write!(fmt, "Actix error"),
        }
    }
}
