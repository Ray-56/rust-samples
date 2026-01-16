use crate::application::auth_service::AuthService;
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // 从 app_data 中获取 AuthService（确保在 App::new() 中注入了 AuthService）
    let auth_service = match req.app_data::<web::Data<AuthService>>() {
        Some(data) => data,
        None => return Err((ErrorUnauthorized("AuthService not found"), req)),
    };

    let token = credentials.token();
    match auth_service.validate_token(token) {
        Ok(claims) => {
            // 将验证后的 Claims 插入到请求中供后续 handler 使用
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            // 返回 401 Unauthorized 错误
            Err((ErrorUnauthorized("Invalid token"), req))
        }
    }
}
