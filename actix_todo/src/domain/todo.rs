use std::{
    fmt::{self, Display},
    str::FromStr,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct TodoItem {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "Todo item description")]
    pub description: String,
    #[serde(default = "default_status")]
    #[schema(example = "pending")]
    pub status: TodoStatus,
    #[serde(default)]
    #[schema(example = 1000)]
    pub position: usize,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "2023-10-01T12:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub enum TodoStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "doing")]
    Doing,
}

impl FromStr for TodoStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(TodoStatus::Pending),
            "completed" => Ok(TodoStatus::Completed),
            "doing" => Ok(TodoStatus::Doing),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "pending"),
            TodoStatus::Doing => write!(f, "doing"),
            TodoStatus::Completed => write!(f, "completed"),
        }
    }
}

fn default_status() -> TodoStatus {
    TodoStatus::Pending
}

