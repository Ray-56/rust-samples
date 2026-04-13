use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

use crate::interfaces::{account_handler, auth_handler, todo_handler};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handler::login,
        account_handler::register,
        account_handler::reset_password,
        account_handler::reset_password_by_username,
        account_handler::update_account,
        account_handler::get_accounts,
        todo_handler::get_todos,
        todo_handler::add_todo,
        todo_handler::update_todo,
        todo_handler::patch_todo,
        todo_handler::delete_todo,
        todo_handler::reorder_todos,
    ),
    modifiers(
        &SecurityAddon
    ),
    tags(
        (name = "Account API", description = "API for managing user accounts"),
        (name = "Auth API", description = "API for authentication"),
        (name = "Todo API", description = "API for managing Todo List"),
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub struct ApiDoc;

// Custom OpenAPI modifier, add securitySchemes
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .get_or_insert(utoipa::openapi::Components::new());
        components.security_schemes.insert(
            "BearerAuth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
