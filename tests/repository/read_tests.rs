use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::domain::repository::task::{TaskRepository, TaskError};

#[tokio::test]
async fn test_in_memory_repository_not_found() {
    let repo = InMemoryTaskRepository::new();
    
    // 存在しないタスクを取得しようとするとエラー
    let result = repo.get_by_id(999).await;
    assert!(matches!(result, Err(TaskError::NotFound(999))));
} 