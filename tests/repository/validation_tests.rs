use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::domain::model::task::CreateTask;
use todo_api::domain::repository::task::{TaskRepository, TaskError};

#[tokio::test]
async fn test_in_memory_repository_validation() {
    let repo = InMemoryTaskRepository::new();
    
    // 空の説明でタスクを作成しようとするとエラー
    let create_task = CreateTask {
        description: "".to_string(),
    };
    let result = repo.create(create_task).await;
    assert!(matches!(result, Err(TaskError::ValidationError(_))));
} 