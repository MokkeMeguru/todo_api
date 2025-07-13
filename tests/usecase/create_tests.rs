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

#[tokio::test]
async fn test_create_task_with_validation() {
    let create_task = CreateTask {
        description: "Valid task".to_string(),
    };
    let created_task = Task::new(1, "Valid task".to_string()).unwrap();
    
    let mut mock_repo = MockTaskRepository::default();
    mock_repo.expect_create()
        .times(1)
        .returning(move |_| Ok(created_task.clone()));
    let usecase = TaskUsecase::new(mock_repo);
    
    let result = usecase.create_task(create_task).await;
    assert!(result.is_ok());
} 