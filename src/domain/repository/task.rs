use async_trait::async_trait;
use thiserror::Error;
use crate::domain::model::task::{Task, CreateTask, UpdateTask, TaskValidationError};

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found with id {0}")]
    NotFound(u64),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Validation error: {0}")]
    ValidationError(#[from] TaskValidationError),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Task>, TaskError>;
    async fn get_by_id(&self, id: u64) -> Result<Task, TaskError>;
    async fn create(&self, task: CreateTask) -> Result<Task, TaskError>;
    async fn update(&self, id: u64, update_task: UpdateTask) -> Result<Task, TaskError>;
    async fn delete(&self, id: u64) -> Result<(), TaskError>;
    async fn complete(&self, id: u64) -> Result<Task, TaskError>;
    async fn uncomplete(&self, id: u64) -> Result<Task, TaskError>;
} 