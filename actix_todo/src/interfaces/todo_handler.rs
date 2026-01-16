use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use log::info;

use crate::application::todo_service::TodoService;
use crate::domain::todo::TodoItem;

use super::dtos::{AddTodoItem, PatchTodoItem, PutTodoItem, ReorderTodoItem};

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "Get all todos", body = [TodoItem]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todo API"
)]
#[get("/todos")]
pub async fn get_todos(service: web::Data<TodoService>) -> impl Responder {
    match service.get_todos().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = AddTodoItem,
    responses(
        (status = 200, description = "Add a new todo", body = TodoItem),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todo API"
)]
#[post("/todos")]
pub async fn add_todo(
    service: web::Data<TodoService>,
    item: web::Json<AddTodoItem>,
) -> impl Responder {
    match service.add_todo(item.into_inner()).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    patch,
    path = "/todos/{id}",
    request_body = PatchTodoItem,
    responses(
        (status = 200, description = "Update a todo", body = TodoItem),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todo API"
)]
#[patch("/todos/{id}")]
pub async fn patch_todo(
    service: web::Data<TodoService>,
    id: web::Path<i32>,
    item: web::Json<PatchTodoItem>,
) -> impl Responder {
    match service.patch_todo(id.into_inner(), item.into_inner()).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            err => {
                info!("Error: patch_todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = PutTodoItem,
    responses(
        (status = 200, description = "Update a todo", body = TodoItem),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todo API"
)]
#[put("/todos/{id}")]
pub async fn update_todo(
    service: web::Data<TodoService>,
    id: web::Path<i32>,
    item: web::Json<PutTodoItem>,
) -> impl Responder {
    match service
        .update_todo(id.into_inner(), item.into_inner())
        .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 204, description = "Delete a todo"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todo API"
)]
#[delete("/todos/{id}")]
pub async fn delete_todo(service: web::Data<TodoService>, id: web::Path<i32>) -> impl Responder {
    match service.delete_todo(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("Error: {:?}", e);
            info!("Error: delete_todo: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        } // Err(e) => match e {
          //     sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
          //     err => {
          //         println!("Error: {:?}", err);
          //         info!("Error: delete_todo: {:?}", err);
          //         HttpResponse::InternalServerError().finish()
          //     }
          // },
    }
}

#[utoipa::path(
    patch,
    path = "/todos/reorder",
    request_body = ReorderTodoItem,
    responses(
        (status = 200, description = "Reorder todos", body = [TodoItem]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todo API"
)]
#[patch("/todos/reorder")]
pub async fn reorder_todos(
    service: web::Data<TodoService>,
    item: web::Json<ReorderTodoItem>,
) -> impl Responder {
    match service.reorder_todos(item.status.clone()).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            println!("reorganize failed with error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
