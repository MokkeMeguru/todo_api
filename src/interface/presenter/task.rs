use crate::domain::model::task::{Task, CreateTask, UpdateTask};
use openapi::models::{Task as ApiTask, CreateTask as ApiCreateTask, UpdateTask as ApiUpdateTask};

/// ドメインモデルとAPIモデル間の変換を行うマッパー
pub struct TaskMapper;

impl TaskMapper {
    /// ドメインのTaskをAPIのTaskに変換
    pub fn domain_to_api(domain_task: Task) -> ApiTask {
        ApiTask {
            id: domain_task.id as i64, // u64 -> i64 変換
            description: domain_task.description,
            completed: domain_task.completed,
            created_at: domain_task.created_at,
            updated_at: domain_task.updated_at,
        }
    }

    /// APIのTaskをドメインのTaskに変換
    pub fn api_to_domain(api_task: ApiTask) -> Task {
        Task {
            id: api_task.id as u64, // i64 -> u64 変換
            description: api_task.description,
            completed: api_task.completed,
            created_at: api_task.created_at,
            updated_at: api_task.updated_at,
        }
    }

    /// ドメインのCreateTaskをAPIのCreateTaskに変換
    pub fn domain_create_to_api(domain_create: CreateTask) -> ApiCreateTask {
        ApiCreateTask {
            description: domain_create.description,
        }
    }

    /// APIのCreateTaskをドメインのCreateTaskに変換
    pub fn api_create_to_domain(api_create: ApiCreateTask) -> Result<CreateTask, crate::domain::model::task::TaskValidationError> {
        CreateTask::new(api_create.description)
    }

    /// ドメインのUpdateTaskをAPIのUpdateTaskに変換
    pub fn domain_update_to_api(domain_update: UpdateTask) -> ApiUpdateTask {
        ApiUpdateTask {
            description: domain_update.description,
            completed: domain_update.completed,
        }
    }

    /// APIのUpdateTaskをドメインのUpdateTaskに変換
    pub fn api_update_to_domain(api_update: ApiUpdateTask) -> Result<UpdateTask, crate::domain::model::task::TaskValidationError> {
        UpdateTask::new(api_update.description, api_update.completed)
    }

    /// ドメインのTaskのベクターをAPIのTaskのベクターに変換
    pub fn domain_vec_to_api(domain_tasks: Vec<Task>) -> Vec<ApiTask> {
        domain_tasks.into_iter().map(Self::domain_to_api).collect()
    }

    /// APIのTaskのベクターをドメインのTaskのベクターに変換
    pub fn api_vec_to_domain(api_tasks: Vec<ApiTask>) -> Vec<Task> {
        api_tasks.into_iter().map(Self::api_to_domain).collect()
    }
} 