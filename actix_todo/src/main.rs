use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use application::{
    account_service::AccountService, auth_service::AuthService, todo_service::TodoService,
};
use dotenv::dotenv;
use infrastructure::db::{establish_connection, run_migrations};
use interfaces::{
    account_handler, api_doc, auth_handler, middleware::jwt_middleware, todo_handler,
};
use log::info;
use std::env;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

mod application;
mod domain;
mod infrastructure;
mod interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let db_pool = establish_connection().await;
    run_migrations(&db_pool).await;

    let todo_service = web::Data::new(TodoService {
        db_pool: db_pool.clone(),
    });
    let jwt_secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let auth_service = web::Data::new(AuthService::new(jwt_secret_key));
    let account_service = web::Data::new(AccountService {
        db_pool: db_pool.clone(),
    });

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        let auth = HttpAuthentication::bearer(jwt_middleware);

        App::new()
            .app_data(todo_service.clone())
            .app_data(auth_service.clone())
            .app_data(account_service.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()),
            )
            .service(Redoc::with_url("/redoc", api_doc::ApiDoc::openapi()))
            .service(auth_handler::login)
            .service(account_handler::register)
            .service(account_handler::reset_password_by_username)
            .service(
                web::scope("")
                    .wrap(auth) // protected route
                    .service(account_handler::update_account)
                    .service(account_handler::get_accounts)
                    .service(account_handler::reset_password)
                    .service(todo_handler::reorder_todos)
                    .service(todo_handler::get_todos)
                    .service(todo_handler::add_todo)
                    .service(todo_handler::update_todo)
                    .service(todo_handler::patch_todo)
                    .service(todo_handler::delete_todo),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
