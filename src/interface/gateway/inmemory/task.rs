use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::domain::model::task::{Task, CreateTask, UpdateTask};
use crate::domain::repository::task::{TaskRepository, TaskError};

#[derive(Clone)]
pub struct InMemoryTaskRepository {
    tasks: Arc<Mutex<HashMap<u64, Task>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn get_all(&self) -> Result<Vec<Task>, TaskError> {
        let tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        Ok(tasks.values().cloned().collect())
    }

    async fn get_by_id(&self, id: u64) -> Result<Task, TaskError> {
        let tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        tasks.get(&id)
            .cloned()
            .ok_or(TaskError::NotFound(id))
    }

    async fn create(&self, create_task: CreateTask) -> Result<Task, TaskError> {
        // バリデーション
        create_task.validate()?;

        let mut tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        let mut next_id = self.next_id.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        let task = Task::new(*next_id, create_task.description)?;
        tasks.insert(*next_id, task.clone());
        *next_id += 1;
        
        Ok(task)
    }

    async fn update(&self, id: u64, update_task: UpdateTask) -> Result<Task, TaskError> {
        if update_task.is_empty() {
            return Err(TaskError::InvalidOperation("Update task cannot be empty".to_string()));
        }

        // バリデーション
        update_task.validate()?;

        let mut tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        let mut task = tasks.get(&id)
            .cloned()
            .ok_or(TaskError::NotFound(id))?;
        
        // 部分更新の適用
        if let Some(description) = update_task.description {
            task.update_description(description)?;
        }
        
        if let Some(completed) = update_task.completed {
            if completed {
                task.complete();
            } else {
                task.uncomplete();
            }
        }
        
        tasks.insert(id, task.clone());
        Ok(task)
    }

    async fn delete(&self, id: u64) -> Result<(), TaskError> {
        let mut tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        if tasks.remove(&id).is_some() {
            Ok(())
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    async fn complete(&self, id: u64) -> Result<Task, TaskError> {
        let mut tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        let mut task = tasks.get(&id)
            .cloned()
            .ok_or(TaskError::NotFound(id))?;
        
        task.complete();
        tasks.insert(id, task.clone());
        Ok(task)
    }

    async fn uncomplete(&self, id: u64) -> Result<Task, TaskError> {
        let mut tasks = self.tasks.lock().map_err(|e| {
            TaskError::RepositoryError(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock: {}", e),
            )))
        })?;
        
        let mut task = tasks.get(&id)
            .cloned()
            .ok_or(TaskError::NotFound(id))?;
        
        task.uncomplete();
        tasks.insert(id, task.clone());
        Ok(task)
    }
} 