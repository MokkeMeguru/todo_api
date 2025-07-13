use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::domain::model::task::CreateTask;
use todo_api::domain::repository::task::TaskRepository;

#[tokio::test]
async fn test_in_memory_repository_create_and_get() {
    let repo = InMemoryTaskRepository::new();
    
    // タスクを作成
    let create_task = CreateTask {
        description: "Test task".to_string(),
    };
    let created_task = repo.create(create_task).await.unwrap();
    
    // 作成されたタスクを取得
    let retrieved_task = repo.get_by_id(created_task.id).await.unwrap();
    
    assert_eq!(created_task.id, retrieved_task.id);
    assert_eq!(created_task.description, retrieved_task.description);
    assert_eq!(created_task.completed, retrieved_task.completed);
}

#[tokio::test]
async fn test_in_memory_repository_get_all() {
    let repo = InMemoryTaskRepository::new();
    
    // 複数のタスクを作成
    let task1 = CreateTask {
        description: "Task 1".to_string(),
    };
    let task2 = CreateTask {
        description: "Task 2".to_string(),
    };
    
    repo.create(task1).await.unwrap();
    repo.create(task2).await.unwrap();
    
    let all_tasks = repo.get_all().await.unwrap();
    assert_eq!(all_tasks.len(), 2);
} 