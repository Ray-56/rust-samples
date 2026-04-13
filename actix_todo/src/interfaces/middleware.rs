use crate::application::auth_service::AuthService;
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Get the AuthService from app_data (make sure AuthService is injected in App::new())
    let auth_service = match req.app_data::<web::Data<AuthService>>() {
        Some(data) => data,
        None => return Err((ErrorUnauthorized("AuthService not found"), req)),
    };

    let token = credentials.token();
    match auth_service.validate_token(token) {
        Ok(claims) => {
            // Insert the verified Claims into the request for use by subsequent handlers
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            // Returns 401 Unauthorized error
            Err((ErrorUnauthorized("Invalid token"), req))
        }
    }
}
