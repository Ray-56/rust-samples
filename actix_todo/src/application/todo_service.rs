use crate::domain::todo::{AddTodoItem, PatchTodoItem, PutTodoItem, TodoItem, TodoStatus};
use sqlx::MySqlPool;
use log::{info, error};

pub struct TodoService {
    pub db_pool: MySqlPool,
}

impl TodoService {
    pub async fn get_todos(&self) -> Result<Vec<TodoItem>, sqlx::Error> {
        info!("Fetching all todos");
        let query = sqlx::query!(
            r#"
            SELECT id, description, status, position, created_at, updated_at
            FROM todos
            ORDER BY position
            "#
        );

        let result = query.fetch_all(&self.db_pool).await?;
        let todos = result
            .iter()
            .map(|row| TodoItem {
                id: row.id as usize,
                description: row.description.clone(),
                status: match row.status.as_str() {
                    "pending" => TodoStatus::Pending,
                    "completed" => TodoStatus::Completed,
                    "doing" => TodoStatus::Doing,
                    _ => TodoStatus::Pending,
                },
                position: row.position as usize,
                created_at: row.created_at.map(|dt| dt.unix_timestamp()).unwrap_or(0),
                updated_at: row.updated_at.map(|dt| dt.unix_timestamp()).unwrap_or(0),
            })
            .collect();
        Ok(todos)
    }

    pub async fn add_todo(&self, item: AddTodoItem) -> Result<TodoItem, sqlx::Error> {
        info!("Adding new todo: {:?}", item);
        let description = item.description.unwrap_or_default();
        let max_position = sqlx::query!(
            r#"
            SELECT MAX(position) as position
            FROM todos
            WHERE status = 'pending'
            "#
        )
        .fetch_one(&self.db_pool)
        .await?
        .position
        .unwrap_or(1000);

        let position = max_position + 1000;

        let query = sqlx::query!(
            r#"
			INSERT INTO todos (description, status, position)
			VALUES (?, ?, ?)
			"#,
            description,
            "pending",
            position
        );

        let result = query.execute(&self.db_pool).await?;
        let id = result.last_insert_id() as usize;
        let todo = self.get_todo_by_id(id).await?;
        Ok(todo)
    }

    pub async fn get_todo_by_id(&self, id: usize) -> Result<TodoItem, sqlx::Error> {
        info!("Fetching todo by id: {}", id);
        let query = sqlx::query!(r#"SELECT * FROM todos WHERE id = ?"#, id as i64);

        match query.fetch_one(&self.db_pool).await {
            Ok(result) => Ok(TodoItem {
                id: result.id as usize,
                description: result.description.clone(),
                status: match result.status.as_str() {
                    "pending" => TodoStatus::Pending,
                    "completed" => TodoStatus::Completed,
                    "doing" => TodoStatus::Doing,
                    _ => TodoStatus::Pending,
                },
                position: result.position as usize,
                created_at: result.created_at.map(|dt| dt.unix_timestamp()).unwrap_or(0),
                updated_at: result.updated_at.map(|dt| dt.unix_timestamp()).unwrap_or(0),
            }),
            Err(sqlx::Error::RowNotFound) => {
                error!("Todo not found with id: {}", id);
                Err(sqlx::Error::RowNotFound)
            },
            Err(e) => {
                error!("Error fetching todo by id: {}", e);
                Err(e)
            },
        }
    }

    pub async fn patch_todo(
        &self,
        id: usize,
        item: PatchTodoItem,
    ) -> Result<TodoItem, sqlx::Error> {
        info!("Patching todo with id: {}", id);
        let mut todo = self.get_todo_by_id(id).await?;
        if let Some(description) = item.description {
            todo.description = description;
        }
        if let Some(status) = item.status {
            todo.status = status;
        }
        if let Some(position) = item.position {
            todo.position = position;
        }
        let query = sqlx::query!(
            r#"
			UPDATE todos
			SET description = ?, status = ?, position = ?
			WHERE id = ?
			"#,
            todo.description,
            todo.status.as_str(),
            todo.position as i64,
            todo.id as i64
        );
        query.execute(&self.db_pool).await?;
        let updated_todo = self.get_todo_by_id(id).await?;
        Ok(updated_todo)
    }

    pub async fn update_todo(&self, id: usize, todo: PutTodoItem) -> Result<TodoItem, sqlx::Error> {
        info!("Updating todo with id: {}", id);
        let query = sqlx::query!(
            r#"
            UPDATE todos
            SET description = ?, status = ?, position = ?
            WHERE id = ?
            "#,
            todo.description,
            todo.status.as_str(),
            todo.position as i64,
            id as i64
        );

        query.execute(&self.db_pool).await?;
        let todo = self.get_todo_by_id(id).await?;
        Ok(todo)
    }

    pub async fn delete_todo(&self, id: usize) -> Result<(), sqlx::Error> {
        info!("Deleting todo with id: {}", id);
        let query = sqlx::query!(
            r#"
			DELETE FROM todos
			WHERE id = ?
			"#,
            id as i64
        );
        query.execute(&self.db_pool).await?;
        Ok(())
    }

    /// Reorder todos by status
    pub async fn reorder_todos(&self, status: TodoStatus) -> Result<Vec<TodoItem>, sqlx::Error> {
        info!("Reordering todos by status: {:?}", status);
        let query = sqlx::query!(
            r#"
            UPDATE todos
            JOIN (
                SELECT id, ROW_NUMBER() OVER (ORDER BY position ASC) * 1000 AS new_position
                FROM todos
                WHERE status = ?
            ) AS subquery
            ON todos.id = subquery.id
            SET todos.position = subquery.new_position
            "#,
            status.as_str()
        );

        query.execute(&self.db_pool).await?;
        let todos = self.get_todos().await?;
        Ok(todos)
    }
}
