use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::domain::model::task::CreateTask;
use todo_api::domain::repository::task::{TaskRepository, TaskError};

#[tokio::test]
async fn test_in_memory_repository_delete() {
    let repo = InMemoryTaskRepository::new();
    
    // タスクを作成
    let create_task = CreateTask {
        description: "Task to delete".to_string(),
    };
    let created_task = repo.create(create_task).await.unwrap();
    
    // タスクを削除
    let delete_result = repo.delete(created_task.id).await;
    assert!(delete_result.is_ok());
    
    // 削除されたタスクを取得しようとするとエラー
    let get_result = repo.get_by_id(created_task.id).await;
    assert!(matches!(get_result, Err(TaskError::NotFound(_))));
} 