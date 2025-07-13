use crate::domain::model::task::{CreateTask, Task, UpdateTask};
use crate::domain::repository::task::TaskRepository;
use crate::domain::model::task::TaskValidationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found with id: {0}")]
    NotFound(u64),
    #[error("Validation error: {0}")]
    Validation(#[from] TaskValidationError),
    #[error("Repository error: {0}")]
    Repository(String),
}

pub trait TaskUsecase: Send + Sync {
    fn get_all_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>>;
    fn get_task_by_id<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>>;
    fn create_task<'a>(&'a self, create_task: CreateTask) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>>;
    fn update_task<'a>(&'a self, id: u64, update_task: UpdateTask) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>>;
    fn delete_task<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TaskError>> + Send + 'a>>;
    fn complete_task<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>>;
    fn uncomplete_task<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>>;
    fn get_completed_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>>;
    fn get_pending_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>>;
    fn search_tasks<'a>(&'a self, query: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>>;
    fn get_tasks_by_status<'a>(&'a self, completed: bool) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>>;
}

pub struct TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + 'static,
{
    repository: R,
}

impl<R> TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + 'static,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, TaskError> {
        self.repository.get_all().await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn get_task_by_id(&self, id: u64) -> Result<Task, TaskError> {
        self.repository.get_by_id(id).await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn create_task(&self, create_task: CreateTask) -> Result<Task, TaskError> {
        create_task.validate()?;
        self.repository.create(create_task).await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn update_task(&self, id: u64, update_task: UpdateTask) -> Result<Task, TaskError> {
        update_task.validate()?;
        self.repository.update(id, update_task).await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn delete_task(&self, id: u64) -> Result<(), TaskError> {
        // Check if task exists before deleting
        self.repository.get_by_id(id).await.map_err(|e| TaskError::Repository(e.to_string()))?;
        self.repository.delete(id).await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn complete_task(&self, id: u64) -> Result<Task, TaskError> {
        self.repository.complete(id).await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn uncomplete_task(&self, id: u64) -> Result<Task, TaskError> {
        self.repository.uncomplete(id).await.map_err(|e| TaskError::Repository(e.to_string()))
    }

    pub async fn get_completed_tasks(&self) -> Result<Vec<Task>, TaskError> {
        let all_tasks = self.repository.get_all().await.map_err(|e| TaskError::Repository(e.to_string()))?;
        Ok(all_tasks.into_iter().filter(|t| t.completed).collect())
    }

    pub async fn get_pending_tasks(&self) -> Result<Vec<Task>, TaskError> {
        let all_tasks = self.repository.get_all().await.map_err(|e| TaskError::Repository(e.to_string()))?;
        Ok(all_tasks.into_iter().filter(|t| !t.completed).collect())
    }

    pub async fn search_tasks(&self, query: &str) -> Result<Vec<Task>, TaskError> {
        let all_tasks = self.repository.get_all().await.map_err(|e| TaskError::Repository(e.to_string()))?;
        let filtered_tasks: Vec<Task> = all_tasks
            .into_iter()
            .filter(|task| {
                task.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect();
        Ok(filtered_tasks)
    }

    pub async fn get_tasks_by_status(&self, completed: bool) -> Result<Vec<Task>, TaskError> {
        let all_tasks = self.repository.get_all().await.map_err(|e| TaskError::Repository(e.to_string()))?;
        Ok(all_tasks.into_iter().filter(|t| t.completed == completed).collect())
    }
}

impl<R> Clone for TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + Clone + 'static,
{
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
        }
    }
}

impl<R> TaskUsecase for TaskUsecaseImpl<R>
where
    R: TaskRepository + Send + Sync + 'static,
{
    fn get_all_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>> {
        Box::pin(self.get_all_tasks())
    }
    fn get_task_by_id<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>> {
        Box::pin(self.get_task_by_id(id))
    }
    fn create_task<'a>(&'a self, create_task: CreateTask) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>> {
        Box::pin(self.create_task(create_task))
    }
    fn update_task<'a>(&'a self, id: u64, update_task: UpdateTask) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>> {
        Box::pin(self.update_task(id, update_task))
    }
    fn delete_task<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TaskError>> + Send + 'a>> {
        Box::pin(self.delete_task(id))
    }
    fn complete_task<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>> {
        Box::pin(self.complete_task(id))
    }
    fn uncomplete_task<'a>(&'a self, id: u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Task, TaskError>> + Send + 'a>> {
        Box::pin(self.uncomplete_task(id))
    }
    fn get_completed_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>> {
        Box::pin(self.get_completed_tasks())
    }
    fn get_pending_tasks<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>> {
        Box::pin(self.get_pending_tasks())
    }
    fn search_tasks<'a>(&'a self, query: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>> {
        Box::pin(self.search_tasks(query))
    }
    fn get_tasks_by_status<'a>(&'a self, completed: bool) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Task>, TaskError>> + Send + 'a>> {
        Box::pin(self.get_tasks_by_status(completed))
    }
} 