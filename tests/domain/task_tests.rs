use todo_api::domain::model::task::{Task, CreateTask, UpdateTask, TaskValidationError};

#[test]
fn test_task_creation() {
    let task = Task::new(1, "Test task".to_string()).unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, "Test task");
    assert!(!task.completed);
    assert!(task.is_pending());
}

#[test]
fn test_task_validation_empty_description() {
    let result = Task::new(1, "".to_string());
    assert!(matches!(result, Err(TaskValidationError::EmptyDescription)));
}

#[test]
fn test_task_validation_whitespace_description() {
    let result = Task::new(1, "   ".to_string());
    assert!(matches!(result, Err(TaskValidationError::EmptyDescription)));
}

#[test]
fn test_task_validation_description_too_long() {
    let long_description = "a".repeat(1001);
    let result = Task::new(1, long_description);
    assert!(matches!(result, Err(TaskValidationError::DescriptionTooLong(1000))));
}

#[test]
fn test_task_complete() {
    let mut task = Task::new(1, "Test task".to_string()).unwrap();
    task.complete();
    assert!(task.completed);
    assert!(task.is_completed());
    assert!(!task.is_pending());
}

#[test]
fn test_task_uncomplete() {
    let mut task = Task::new(1, "Test task".to_string()).unwrap();
    task.complete();
    task.uncomplete();
    assert!(!task.completed);
    assert!(!task.is_completed());
    assert!(task.is_pending());
}

#[test]
fn test_task_update_description() {
    let mut task = Task::new(1, "Original task".to_string()).unwrap();
    let original_updated_at = task.updated_at;
    
    task.update_description("Updated task".to_string()).unwrap();
    assert_eq!(task.description, "Updated task");
    assert!(task.updated_at > original_updated_at);
}

#[test]
fn test_task_update_description_validation() {
    let mut task = Task::new(1, "Original task".to_string()).unwrap();
    let result = task.update_description("".to_string());
    assert!(matches!(result, Err(TaskValidationError::EmptyDescription)));
    assert_eq!(task.description, "Original task"); // Should remain unchanged
}

#[test]
fn test_create_task_validation() {
    let create_task = CreateTask::new("Valid task".to_string()).unwrap();
    assert_eq!(create_task.description, "Valid task");

    let result = CreateTask::new("".to_string());
    assert!(matches!(result, Err(TaskValidationError::EmptyDescription)));
}

#[test]
fn test_update_task_validation() {
    let update_task = UpdateTask::new(Some("Valid task".to_string()), Some(true)).unwrap();
    assert_eq!(update_task.description, Some("Valid task".to_string()));
    assert_eq!(update_task.completed, Some(true));

    let result = UpdateTask::new(Some("".to_string()), None);
    assert!(matches!(result, Err(TaskValidationError::EmptyDescription)));
}

#[test]
fn test_update_task_is_empty() {
    let empty_update = UpdateTask::new(None, None).unwrap();
    assert!(empty_update.is_empty());

    let non_empty_update = UpdateTask::new(Some("Task".to_string()), None).unwrap();
    assert!(!non_empty_update.is_empty());
} 