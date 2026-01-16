use chrono::Utc;
use log::{error, info};
use sqlx::PgPool;
use std::str::FromStr;

use crate::{
    domain::todo::{TodoItem, TodoStatus},
    interfaces::dtos::{AddTodoItem, PatchTodoItem, PutTodoItem},
};

pub struct TodoService {
    pub db_pool: PgPool,
}

impl TodoService {
    pub async fn get_todos(&self) -> Result<Vec<TodoItem>, sqlx::Error> {
        info!("Fetching all todos");
        let result = sqlx::query!(
            r#"
            SELECT id, description, status, position, created_at, updated_at
            FROM todos
            ORDER BY position
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        let todos = result
            .iter()
            .map(|row| TodoItem {
                id: row.id,
                description: row.description.clone(),
                status: TodoStatus::from_str(row.status.as_str())
                    .unwrap_or_else(|_| TodoStatus::Pending),
                position: row.position as usize,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect::<Vec<TodoItem>>();

        println!("Todos: {:?}", todos);
        Ok(todos)
    }

    pub async fn add_todo(&self, item: AddTodoItem) -> Result<TodoItem, sqlx::Error> {
        info!("Adding new todo: {:?}", item);
        let description = item.description.unwrap_or_default();
        let max_position = sqlx::query!(
            r#"
            SELECT COALESCE(MAX(position), 0) as max_position
            FROM todos
            WHERE status = 'pending'
            "#
        )
        .fetch_one(&self.db_pool)
        .await?
        .max_position
        .unwrap_or(1000);

        let position = max_position + 1000;

        let query = sqlx::query!(
            r#"
            INSERT INTO todos (description, status, position)
            VALUES ($1, $2, $3)
            RETURNING id, description, status, position, created_at, updated_at
            "#,
            description,
            "pending",
            position
        );

        let result = query.fetch_one(&self.db_pool).await?;
        let todo = TodoItem {
            id: result.id,
            description: result.description.clone(),
            status: TodoStatus::from_str(result.status.as_str())
                .unwrap_or_else(|_| TodoStatus::Pending),
            position: result.position as usize,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        println!("Added todo: {:?}", todo);
        Ok(todo)
    }

    pub async fn patch_todo(&self, id: i32, item: PatchTodoItem) -> Result<TodoItem, sqlx::Error> {
        info!("Patching todo with id {}", id);
        let query = sqlx::query!(
            r#"
            UPDATE todos
            SET description = COALESCE($1, description),
                status = COALESCE($2, status),
                position = COALESCE($3, position)
            WHERE id = $4
            RETURNING id, description, status, position, created_at, updated_at
            "#,
            item.description,
            item.status.map(|s| s.as_str().to_owned()),
            item.position.map(|p| p as i32),
            id
        );
        let result = query.fetch_one(&self.db_pool).await?;
        let todo = TodoItem {
            id: result.id,
            description: result.description.clone(),
            status: TodoStatus::from_str(result.status.as_str())
                .unwrap_or_else(|_| TodoStatus::Pending),
            position: result.position as usize,
            created_at: result.created_at,
            updated_at: result.updated_at,
        };
        println!("Patched todo: {:?}", todo);
        Ok(todo)
    }

    pub async fn update_todo(&self, id: i32, todo: PutTodoItem) -> Result<TodoItem, sqlx::Error> {
        info!("Updating todo with id {}", id);
        let query = sqlx::query!(
            r#"
            UPDATE todos
            SET description = $1,
                status = $2,
                position = $3
            WHERE id = $4
            RETURNING id, description, status, position, created_at, updated_at
            "#,
            todo.description,
            todo.status.as_str(),
            todo.position as i32,
            id
        );
        let result = query.fetch_one(&self.db_pool).await?;
        let todo = TodoItem {
            id: result.id,
            description: result.description.clone(),
            status: TodoStatus::from_str(result.status.as_str())
                .unwrap_or_else(|_| TodoStatus::Pending),
            position: result.position as usize,
            created_at: result.created_at,
            updated_at: result.updated_at,
        };
        println!("Updated todo: {:?}", todo);
        Ok(todo)
    }

    pub async fn delete_todo(&self, id: i32) -> Result<(), sqlx::Error> {
        info!("Deleting todo with id {}", id);
        let query = sqlx::query!(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
            id
        );
        match query.execute(&self.db_pool).await {
            Ok(rst) => {
                if rst.rows_affected() == 0 {
                    error!("Todo with id {} not found", id);
                    return Err(sqlx::Error::RowNotFound);
                }
                println!("Deleted todo with id: {}", id);
                Ok(())
            }
            Err(e) => {
                error!("Error deleting todo with id {}: {:?}", id, e);
                return Err(e);
            }
        }
    }

    /// Reorder todos by status
    pub async fn reorder_todos(&self, status: TodoStatus) -> Result<Vec<TodoItem>, sqlx::Error> {
        info!("Reordering todos with status: {:?}", status);
        let query = sqlx::query!(
            r#"
            WITH ranked_todos AS (
                SELECT id, ROW_NUMBER() OVER (ORDER BY position ASC) * 1000 AS new_position
                FROM todos
                WHERE status = $1
            )
            UPDATE todos
            SET position = ranked_todos.new_position
            FROM ranked_todos
            WHERE todos.id = ranked_todos.id
            RETURNING todos.id, todos.description, todos.status, todos.position::int, todos.created_at, todos.updated_at
            "#,
            status.as_str()
        );
        let result = query.fetch_all(&self.db_pool).await?;
        let todos = result
            .iter()
            .map(|row| TodoItem {
                id: row.id,
                description: row.description.clone(),
                status: TodoStatus::from_str(row.status.as_str())
                    .unwrap_or_else(|_| TodoStatus::Pending),
                position: row.position as usize,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect::<Vec<TodoItem>>();
        println!("Reordered todos: {:?}", todos);
        Ok(todos)
    }
}
