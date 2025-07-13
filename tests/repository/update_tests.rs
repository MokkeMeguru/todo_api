use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::domain::model::task::{CreateTask, UpdateTask};
use todo_api::domain::repository::task::{TaskRepository, TaskError};

#[tokio::test]
async fn test_in_memory_repository_update() {
    let repo = InMemoryTaskRepository::new();
    
    // タスクを作成
    let create_task = CreateTask {
        description: "Original task".to_string(),
    };
    let created_task = repo.create(create_task).await.unwrap();
    
    // タスクを更新
    let update_task = UpdateTask {
        description: Some("Updated task".to_string()),
        completed: Some(true),
    };
    let result = repo.update(created_task.id, update_task).await.unwrap();
    
    assert_eq!(result.description, "Updated task");
    assert_eq!(result.completed, true);
}

#[tokio::test]
async fn test_in_memory_repository_empty_update() {
    let repo = InMemoryTaskRepository::new();
    
    // タスクを作成
    let create_task = CreateTask {
        description: "Original task".to_string(),
    };
    let created_task = repo.create(create_task).await.unwrap();
    
    // 空の更新を試行
    let update_task = UpdateTask {
        description: None,
        completed: None,
    };
    let result = repo.update(created_task.id, update_task).await;
    assert!(matches!(result, Err(TaskError::InvalidOperation(_))));
}

#[tokio::test]
async fn test_in_memory_repository_complete() {
    let repo = InMemoryTaskRepository::new();
    
    // タスクを作成
    let create_task = CreateTask {
        description: "Task to complete".to_string(),
    };
    let created_task = repo.create(create_task).await.unwrap();
    
    // タスクを完了
    let completed_task = repo.complete(created_task.id).await.unwrap();
    assert!(completed_task.completed);
}

#[tokio::test]
async fn test_in_memory_repository_uncomplete() {
    let repo = InMemoryTaskRepository::new();
    
    // タスクを作成
    let create_task = CreateTask {
        description: "Task to uncomplete".to_string(),
    };
    let created_task = repo.create(create_task).await.unwrap();
    
    // タスクを完了
    let completed_task = repo.complete(created_task.id).await.unwrap();
    assert!(completed_task.completed);
    
    // タスクを未完了に戻す
    let uncompleted_task = repo.uncomplete(created_task.id).await.unwrap();
    assert!(!uncompleted_task.completed);
} 