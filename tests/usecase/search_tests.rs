use todo_api::domain::model::task::{Task, CreateTask, UpdateTask};
use todo_api::domain::repository::task::{TaskRepository, TaskError as DomainTaskError};
use async_trait::async_trait;
use mockall::mock;

mock! {
    pub TaskRepository {}
    #[async_trait]
    impl TaskRepository for TaskRepository {
        async fn get_all(&self) -> Result<Vec<Task>, DomainTaskError>;
        async fn get_by_id(&self, id: u64) -> Result<Task, DomainTaskError>;
        async fn create(&self, task: CreateTask) -> Result<Task, DomainTaskError>;
        async fn update(&self, id: u64, update_task: UpdateTask) -> Result<Task, DomainTaskError>;
        async fn delete(&self, id: u64) -> Result<(), DomainTaskError>;
        async fn complete(&self, id: u64) -> Result<Task, DomainTaskError>;
        async fn uncomplete(&self, id: u64) -> Result<Task, DomainTaskError>;
    }
}

use todo_api::usecase::task::TaskUsecase;
use std::sync::Arc;

#[tokio::test]
async fn test_get_completed_tasks() {
    let task1 = Task::new(1, "Task 1".to_string()).unwrap();
    let task2 = Task::new(2, "Task 2".to_string()).unwrap();
    let mut completed_task = task1.clone();
    completed_task.complete();
    
    let all_tasks = Arc::new(vec![completed_task, task2]);
    
    let mut mock_repo = MockTaskRepository::default();
    let all_tasks_clone = all_tasks.clone();
    mock_repo.expect_get_all()
        .times(1)
        .returning(move || Ok((*all_tasks_clone).clone()));
    let usecase = TaskUsecase::new(mock_repo);
    
    let completed_tasks = usecase.get_completed_tasks().await.unwrap();
    assert_eq!(completed_tasks.len(), 1);
}

#[tokio::test]
async fn test_get_pending_tasks() {
    let task1 = Task::new(1, "Task 1".to_string()).unwrap();
    let task2 = Task::new(2, "Task 2".to_string()).unwrap();
    let mut completed_task = task1.clone();
    completed_task.complete();
    
    let all_tasks = Arc::new(vec![completed_task, task2]);
    
    let mut mock_repo = MockTaskRepository::default();
    let all_tasks_clone = all_tasks.clone();
    mock_repo.expect_get_all()
        .times(1)
        .returning(move || Ok((*all_tasks_clone).clone()));
    let usecase = TaskUsecase::new(mock_repo);
    
    let pending_tasks = usecase.get_pending_tasks().await.unwrap();
    assert_eq!(pending_tasks.len(), 1);
}

#[tokio::test]
async fn test_search_tasks() {
    let task1 = Task::new(1, "Buy groceries".to_string()).unwrap();
    let task2 = Task::new(2, "Clean house".to_string()).unwrap();
    let task3 = Task::new(3, "Buy books".to_string()).unwrap();
    
    let all_tasks = Arc::new(vec![task1, task2, task3]);
    
    let mut mock_repo = MockTaskRepository::default();
    let all_tasks_clone = all_tasks.clone();
    mock_repo.expect_get_all()
        .times(1)
        .returning(move || Ok((*all_tasks_clone).clone()));
    let usecase = TaskUsecase::new(mock_repo);
    
    let search_results = usecase.search_tasks("buy").await.unwrap();
    assert_eq!(search_results.len(), 2);
}

#[tokio::test]
async fn test_get_tasks_by_status() {
    let task1 = Task::new(1, "Task 1".to_string()).unwrap();
    let task2 = Task::new(2, "Task 2".to_string()).unwrap();
    let mut completed_task = task1.clone();
    completed_task.complete();
    
    let all_tasks = Arc::new(vec![completed_task, task2]);
    
    let mut mock_repo = MockTaskRepository::default();
    let all_tasks_clone = all_tasks.clone();
    mock_repo.expect_get_all()
        .times(1)
        .returning(move || Ok((*all_tasks_clone).clone()));
    let usecase = TaskUsecase::new(mock_repo);
    
    let completed_tasks = usecase.get_tasks_by_status(true).await.unwrap();
    assert_eq!(completed_tasks.len(), 1);
    
    let mut mock_repo2 = MockTaskRepository::default();
    let all_tasks_clone2 = all_tasks.clone();
    mock_repo2.expect_get_all()
        .times(1)
        .returning(move || Ok((*all_tasks_clone2).clone()));
    let usecase2 = TaskUsecase::new(mock_repo2);
    
    let pending_tasks = usecase2.get_tasks_by_status(false).await.unwrap();
    assert_eq!(pending_tasks.len(), 1);
} 