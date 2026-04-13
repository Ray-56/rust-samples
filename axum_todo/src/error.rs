use std::convert::Infallible;

use axum::{
    body::{Bytes, Full, Body},
    response::{IntoResponse, Response},
};

/// Application error category
pub enum AppErrorType {
    /// Database error
    DbType,
    /// Resource not found
    NotFount,
}

/// Application error
pub struct AppError {
    /// Error message
    pub message: Option<String>,
    /// Cause from the previous error level
    pub cause: Option<String>,
    /// Error type
    pub error_type: AppErrorType,
}

/// Convert the error into an HTTP response
impl IntoResponse for AppError {
    // type Body = Full<Bytes>;
    // type BodyError = Infallible;

    fn into_response(self) -> Response {
        let msg = match self.message {
            Some(msg) => msg,
            None => "".to_string(),
        };
        msg.into_response()
    }
}
