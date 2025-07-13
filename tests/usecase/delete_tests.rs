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

use todo_api::usecase::task::{TaskUsecase, TaskError};

#[tokio::test]
async fn test_delete_task_with_existence_check() {
    let mut mock_repo = MockTaskRepository::default();
    mock_repo.expect_get_by_id()
        .with(mockall::predicate::eq(999))
        .times(1)
        .returning(|_| Err(todo_api::domain::repository::task::TaskError::NotFound(999)));
    let usecase = TaskUsecase::new(mock_repo);
    
    let result = usecase.delete_task(999).await;
    assert!(matches!(result, Err(TaskError::Repository(_))));
} 