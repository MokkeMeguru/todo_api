use todo_api::domain::model::task::CreateTask;
use todo_api::interface::gateway::inmemory::task::InMemoryTaskRepository;
use todo_api::usecase::task::TaskUsecase;

#[tokio::test]
async fn test_multiple_tasks() {
    let repository = InMemoryTaskRepository::new();
    let usecase = TaskUsecase::new(repository);
    
    // 複数のタスクを作成
    let task1 = CreateTask {
        description: "Task 1".to_string(),
    };
    let task2 = CreateTask {
        description: "Task 2".to_string(),
    };
    let task3 = CreateTask {
        description: "Task 3".to_string(),
    };
    
    let created_task1 = usecase.create_task(task1).await.unwrap();
    let _created_task2 = usecase.create_task(task2).await.unwrap();
    let created_task3 = usecase.create_task(task3).await.unwrap();
    
    // 一部のタスクを完了
    usecase.complete_task(created_task1.id).await.unwrap();
    usecase.complete_task(created_task3.id).await.unwrap();
    
    // すべてのタスクを取得
    let all_tasks = usecase.get_all_tasks().await.unwrap();
    assert_eq!(all_tasks.len(), 3);
    
    // 完了済みタスクを取得
    let completed_tasks = usecase.get_completed_tasks().await.unwrap();
    assert_eq!(completed_tasks.len(), 2);
    
    // 未完了タスクを取得
    let pending_tasks = usecase.get_pending_tasks().await.unwrap();
    assert_eq!(pending_tasks.len(), 1);
    
    // ステータス別にタスクを取得
    let completed_by_status = usecase.get_tasks_by_status(true).await.unwrap();
    assert_eq!(completed_by_status.len(), 2);
    
    let pending_by_status = usecase.get_tasks_by_status(false).await.unwrap();
    assert_eq!(pending_by_status.len(), 1);
} 