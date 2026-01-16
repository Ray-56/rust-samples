use actix_web::{post, web, HttpResponse, Responder};

use crate::application::{account_service::AccountService, auth_service::AuthService};

use super::dtos::{LoginResponse, LoginResquest};

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginResquest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid username or password")
    ),
    security(),
    tag = "Auth API"
)]
#[post("/login")]
pub async fn login(
    auth_service: web::Data<AuthService>,
    account_service: web::Data<AccountService>,
    info: web::Json<LoginResquest>,
) -> impl Responder {
    if let Ok(Some(account)) = account_service.find_by_username(&info.username).await {
        if account.verify_password(&info.password) {
            let token = auth_service.generate_token(&info.username, 3600);
            return HttpResponse::Ok().json(LoginResponse { token });
        }
    }
    HttpResponse::Unauthorized().body("Invalid username or password")
}
