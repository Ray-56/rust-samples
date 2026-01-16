use actix_web::{get, patch, post, web, HttpResponse, Responder};

use crate::application::account_service::AccountService;

use super::dtos::{
    AccountDto, GetAccountsResponse, PaginationRequest, RegisterRequest, RegisterResponse,
    ResetPasswordRequest, ResetPasswordResponse, UpdateAccountRequest, UpdateAccountResponse,
};

#[utoipa::path(
    post,
    path = "/accounts",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Invalid input: username already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(),
    tag = "Account API"
)]
#[post("/accounts")]
pub async fn register(
    account_service: web::Data<AccountService>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    match account_service.register(&req.username, &req.password).await {
        Ok(account) => {
            let response = RegisterResponse {
                id: account.id,
                username: account.username,
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => match e {
            sqlx::Error::Database(_) => {
                HttpResponse::BadRequest().body("Invalid input: username already exists")
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Failed to register user. Error: {}", e)),
        },
    }
}

#[utoipa::path(
    patch,
    path = "/accounts/reset_password/{username}",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully", body = ResetPasswordResponse),
        (status = 400, description = "Invalid input: account not found"),
        (status = 500, description = "Internal server error")
    ),
    security(),
    tag = "Account API"
)]
#[patch("/accounts/reset_password/{username}")]
pub async fn reset_password_by_username(
    account_service: web::Data<AccountService>,
    username: web::Path<String>,
    info: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    match account_service
        .update_password_by_username(&username.into_inner(), &info.new_password)
        .await
    {
        Ok(_) => {
            let response = ResetPasswordResponse {
                message: "Password reset successfully".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::BadRequest().body("Invalid input: account not found")
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Failed to reset password. Error: {}", e)),
        },
    }
}

#[utoipa::path(
    patch,
    path = "/accounts/{account_id}/reset_password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully", body = ResetPasswordResponse),
        (status = 400, description = "Invalid input: account not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Account API"
)]
#[patch("/accounts/{account_id}/reset_password")]
pub async fn reset_password(
    account_service: web::Data<AccountService>,
    account_id: web::Path<i32>,
    info: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    match account_service
        .update_password(account_id.into_inner(), &info.new_password)
        .await
    {
        Ok(_) => {
            let response = ResetPasswordResponse {
                message: "Password reset successfully".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::BadRequest().body("Invalid input: account not found")
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Failed to reset password. Error: {}", e)),
        },
    }
}

#[utoipa::path(
    patch,
    path = "/accounts/{account_id}",
    request_body = UpdateAccountRequest,
    responses(
        (status = 200, description = "Account updated successfully", body = UpdateAccountResponse),
        (status = 400, description = "Invalid input: account not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Account API"
)]
#[patch("/accounts/{account_id}")]
pub async fn update_account(
    account_service: web::Data<AccountService>,
    account_id: web::Path<i32>,
    info: web::Json<UpdateAccountRequest>,
) -> impl Responder {
    match account_service
        .update_account_info(account_id.into_inner(), info.new_username.as_deref())
        .await
    {
        Ok(account) => {
            let response = UpdateAccountResponse {
                account: AccountDto::from(account),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                HttpResponse::BadRequest().body("Invalid input: account not found")
            }
            sqlx::Error::Database(db_err) => {
                if db_err
                    .message()
                    .contains("duplicate key value violates unique constraint")
                {
                    HttpResponse::BadRequest().body("Invalid input: username already exists")
                } else {
                    HttpResponse::InternalServerError()
                        .body(format!("Failed to update account. Error: {}", db_err))
                }
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Failed to update account. Error: {}", e)),
        },
    }
}

#[utoipa::path(
    get,
    path = "/accounts",
    params(
        PaginationRequest
    ),
    responses(
        (status = 200, description = "Get accounts successfully", body = GetAccountsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Account API"
)]
#[get("/accounts")]
pub async fn get_accounts(
    account_service: web::Data<AccountService>,
    info: web::Query<PaginationRequest>,
) -> impl Responder {
    let page = info.page.unwrap_or(1);
    let page_size = info.page_size.unwrap_or(10);
    match account_service
        .get_accounts((page - 1) * page_size, page_size)
        .await
    {
        Ok((accounts, total)) => {
            let accounts: Vec<AccountDto> = accounts.into_iter().map(AccountDto::from).collect();
            HttpResponse::Ok().json(GetAccountsResponse {
                data: accounts,
                total,
            })
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to get accounts. Error: {}", e)),
    }
}
