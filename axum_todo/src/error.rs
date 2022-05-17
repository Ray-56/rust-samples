use std::convert::Infallible;

use axum::{
    body::{Bytes, Full, Body},
    response::{IntoResponse, Response},
};

/// 错误的类型
pub enum AppErrorType {
    /// 数据库错误
    DbType,
    /// 未找到
    NotFount,
}

/// 应用错误
pub struct AppError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: AppErrorType,
}

/// 实现 IntoResponse
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
