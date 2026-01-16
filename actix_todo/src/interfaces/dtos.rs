use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::domain::{account::Account, todo::TodoStatus};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct PaginationRequest {
    #[schema(default = 1, example = 1)]
    pub page: Option<i64>,
    #[schema(default = 10, example = 10)]
    pub page_size: Option<i64>,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginResquest {
    #[schema(example = "admin")]
    pub username: String,
    #[schema(example = "123456")]
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "your_jwt_token")]
    pub token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "testuser")]
    pub username: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "testuser")]
    pub username: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    #[schema(example = "newpassword123")]
    pub new_password: String,
}

#[derive(Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    #[schema(example = "Password reset successfully")]
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AccountDto {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "testuser")]
    pub username: String,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            username: account.username,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct GetAccountsResponse {
    pub data: Vec<AccountDto>,
    pub total: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateAccountRequest {
    #[schema(example = "newusername")]
    pub new_username: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct UpdateAccountResponse {
    pub account: AccountDto,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AddTodoItem {
    #[schema(example = "Todo item description")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PatchTodoItem {
    #[schema(example = "Todo item description")]
    pub description: Option<String>,
    #[schema(example = "pending")]
    pub status: Option<TodoStatus>,
    #[schema(example = 1000)]
    pub position: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct PutTodoItem {
    #[schema(example = "Todo item description")]
    pub description: String,
    #[schema(example = "pending")]
    pub status: TodoStatus,
    #[schema(example = 1000)]
    pub position: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReorderTodoItem {
    #[schema(example = "pending")]
    pub status: TodoStatus,
}
