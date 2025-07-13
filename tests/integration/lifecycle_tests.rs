use todo_api::domain::model::task::{CreateTask, UpdateTask};
use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::usecase::task::TaskUsecase;

#[tokio::test]
async fn test_full_task_lifecycle() {
    // リポジトリとユースケースを作成
    let repository = InMemoryTaskRepository::new();
    let usecase = TaskUsecase::new(repository);
    
    // 1. タスクを作成
    let create_task = CreateTask {
        description: "Test task".to_string(),
    };
    let created_task = usecase.create_task(create_task).await.unwrap();
    assert_eq!(created_task.description, "Test task");
    assert!(!created_task.completed);
    
    // 2. 作成されたタスクを取得
    let retrieved_task = usecase.get_task_by_id(created_task.id).await.unwrap();
    assert_eq!(retrieved_task.id, created_task.id);
    assert_eq!(retrieved_task.description, "Test task");
    
    // 3. タスクを更新
    let update_task = UpdateTask {
        description: Some("Updated task".to_string()),
        completed: Some(true),
    };
    let updated_task = usecase.update_task(created_task.id, update_task).await.unwrap();
    assert_eq!(updated_task.description, "Updated task");
    assert!(updated_task.completed);
    
    // 4. タスクを完了状態に変更
    let completed_task = usecase.complete_task(created_task.id).await.unwrap();
    assert!(completed_task.completed);
    
    // 5. タスクを未完了状態に変更
    let uncompleted_task = usecase.uncomplete_task(created_task.id).await.unwrap();
    assert!(!uncompleted_task.completed);
    
    // 6. すべてのタスクを取得
    let all_tasks = usecase.get_all_tasks().await.unwrap();
    assert_eq!(all_tasks.len(), 1);
    
    // 7. 完了済みタスクを取得
    let completed_tasks = usecase.get_completed_tasks().await.unwrap();
    assert_eq!(completed_tasks.len(), 0); // 未完了状態なので0
    
    // 8. 未完了タスクを取得
    let pending_tasks = usecase.get_pending_tasks().await.unwrap();
    assert_eq!(pending_tasks.len(), 1);
    
    // 9. タスクを検索
    let search_results = usecase.search_tasks("Updated").await.unwrap();
    assert_eq!(search_results.len(), 1);
    
    // 10. タスクを削除
    usecase.delete_task(created_task.id).await.unwrap();
    
    // 11. 削除されたタスクを取得しようとするとエラー
    let result = usecase.get_task_by_id(created_task.id).await;
    assert!(result.is_err());
} 