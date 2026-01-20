use actix_web::{HttpResponse, Result, error, http::StatusCode};
use serde::Serialize;
use std::fmt;

// 没有用到NotFound，rust会发生警告，但是不希望发生警告，所以加上#[allow(dead_code)]
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum WebAppError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct WebAppErrorResponse {
    error_message: String,
}

impl std::error::Error for WebAppError {}

impl WebAppError {
    fn error_response(&self) -> String {
        match self {
            WebAppError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            WebAppError::TeraError(msg) => {
                println!("Error in rendering the template: {:?}", msg);
                msg.into()
            }
            WebAppError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for WebAppError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebAppError::ActixError(_msg) | WebAppError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            WebAppError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let resp = WebAppErrorResponse {
            error_message: self.error_response(),
        };
        HttpResponse::build(self.status_code()).json(resp)
    }
}

impl fmt::Display for WebAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<actix_web::error::Error> for WebAppError {
    fn from(err: actix_web::error::Error) -> Self {
        WebAppError::ActixError(err.to_string())
    }
}
