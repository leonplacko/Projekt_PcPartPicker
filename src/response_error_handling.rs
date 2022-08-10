use actix_web::HttpResponse;
use diesel::result::Error as DBError;
use serde::Serialize;
use serde_json::Error as serdeError;

pub enum ErrorHandler<T> {
    InternalServerError(T),
    BadRequest(T),
    Unauthorized(T),
    NotFound(T),
}

impl<T: Serialize> ErrorHandler<T> {
    pub fn get_response(&self) -> HttpResponse {
        match self {
            ErrorHandler::InternalServerError(message) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(message),
            ErrorHandler::BadRequest(message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(message),
            ErrorHandler::Unauthorized(message) => HttpResponse::Unauthorized()
                .content_type("application/json")
                .json(message),
            ErrorHandler::NotFound(message) => HttpResponse::NotFound()
                .content_type("application/json")
                .json(message),
        }
    }
}

impl From<DBError> for ErrorHandler<String> {
    fn from(error: DBError) -> ErrorHandler<String> {
        match error {
            DBError::NotFound => ErrorHandler::BadRequest(String::from("Data not found")),
            _ => ErrorHandler::InternalServerError(format!(
                "Internal server error (diesel error): {}",
                error.to_string()
            )),
        }
    }
}

impl From<serdeError> for ErrorHandler<String> {
    fn from(error: serdeError) -> Self {
        ErrorHandler::InternalServerError(format!(
            "Internal server error (serde error): {}",
            error.to_string()
        ))
    }
}
