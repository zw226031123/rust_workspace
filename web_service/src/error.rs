use actix_web::{HttpResponse, Result, error, http::StatusCode};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum WebServiceError {
    DBError(String),
    ActixError(String),
    #[allow(dead_code)]
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct WebServiceErrorResponse {
    error_message: String,
}

impl WebServiceError {
    fn error_response(&self) -> String {
        match self {
            WebServiceError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            WebServiceError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            WebServiceError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for WebServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebServiceError::DBError(_msg) | WebServiceError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            WebServiceError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(WebServiceErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for WebServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for WebServiceError {
    fn from(err: actix_web::error::Error) -> Self {
        WebServiceError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for WebServiceError {
    fn from(err: SQLxError) -> Self {
        WebServiceError::DBError(err.to_string())
    }
}
