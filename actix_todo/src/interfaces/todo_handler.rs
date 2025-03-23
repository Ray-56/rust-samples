use crate::application::todo_service::TodoService;
use crate::domain::todo::{AddTodoItem, PatchTodoItem, PutTodoItem, ReorderTodoItem};
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};

#[get("/todos")]
pub async fn get_todos(service: web::Data<TodoService>) -> impl Responder {
    match service.get_todos().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

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

#[patch("/todos/{id}")]
pub async fn patch_todo(
    service: web::Data<TodoService>,
    id: web::Path<usize>,
    item: web::Json<PatchTodoItem>,
) -> impl Responder {
    match service.patch_todo(id.into_inner(), item.into_inner()).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[put("/todos/{id}")]
pub async fn update_todo(
    service: web::Data<TodoService>,
    id: web::Path<usize>,
    item: web::Json<PutTodoItem>,
) -> impl Responder {
    println!("update_todo id: {}, item: {:?}", &id, &item);
    match service
        .update_todo(id.into_inner(), item.into_inner())
        .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(service: web::Data<TodoService>, id: web::Path<usize>) -> impl Responder {
    match service.delete_todo(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

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
