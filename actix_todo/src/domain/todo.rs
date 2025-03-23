use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    #[serde(default = "default_status")]
    pub status: TodoStatus,
    #[serde(default)]
    pub position: usize,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TodoStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "doing")]
    Doing,
}

impl TodoStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TodoStatus::Pending => "pending",
            TodoStatus::Completed => "completed",
            TodoStatus::Doing => "doing",
        }
    }
}

impl Display for TodoStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

fn default_status() -> TodoStatus {
    TodoStatus::Pending
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTodoItem {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PatchTodoItem {
    pub description: Option<String>,
    pub status: Option<TodoStatus>,
    pub position: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutTodoItem {
    pub description: String,
    pub status: TodoStatus,
    pub position: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ReorderTodoItem {
    pub status: TodoStatus,
}
