use todo_api::domain::model::task::{CreateTask, UpdateTask};
use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::usecase::task::TaskUsecase;

#[tokio::test]
async fn test_validation_errors() {
    let repository = InMemoryTaskRepository::new();
    let usecase = TaskUsecase::new(repository);
    
    // 空の説明でタスクを作成しようとするとエラー
    let empty_task = CreateTask {
        description: "".to_string(),
    };
    let result = usecase.create_task(empty_task).await;
    assert!(result.is_err());
    
    // 空白のみの説明でタスクを作成しようとするとエラー
    let whitespace_task = CreateTask {
        description: "   ".to_string(),
    };
    let result = usecase.create_task(whitespace_task).await;
    assert!(result.is_err());
    
    // 長すぎる説明でタスクを作成しようとするとエラー
    let long_description = "a".repeat(1001);
    let long_task = CreateTask {
        description: long_description,
    };
    let result = usecase.create_task(long_task).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_not_found_errors() {
    let repository = InMemoryTaskRepository::new();
    let usecase = TaskUsecase::new(repository);
    
    // 存在しないタスクを取得しようとするとエラー
    let result = usecase.get_task_by_id(999).await;
    assert!(result.is_err());
    
    // 存在しないタスクを更新しようとするとエラー
    let update_task = UpdateTask {
        description: Some("Updated".to_string()),
        completed: None,
    };
    let result = usecase.update_task(999, update_task).await;
    assert!(result.is_err());
    
    // 存在しないタスクを削除しようとするとエラー
    let result = usecase.delete_task(999).await;
    assert!(result.is_err());
    
    // 存在しないタスクを完了しようとするとエラー
    let result = usecase.complete_task(999).await;
    assert!(result.is_err());
    
    // 存在しないタスクを未完了にしようとするとエラー
    let result = usecase.uncomplete_task(999).await;
    assert!(result.is_err());
} 