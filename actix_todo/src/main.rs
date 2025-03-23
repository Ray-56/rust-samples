use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use application::todo_service::TodoService;
use dotenv::dotenv;
use infrastructure::db::{establish_connection, run_migrations};
use interfaces::todo_handler::{
    add_todo, delete_todo, get_todos, reorder_todos, patch_todo, update_todo,
};
// use env_logger::Env;
use log::info;
use log4rs;

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

    let todo_service = web::Data::new(TodoService { db_pool });

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .app_data(todo_service.clone())
            .wrap(cors)
            .service(reorder_todos)
            .service(get_todos)
            .service(add_todo)
            .service(update_todo)
            .service(patch_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
