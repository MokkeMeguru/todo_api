use async_trait::async_trait;
use axum_extra::extract::{CookieJar, Host};
use axum::http::Method;
use std::fmt::Debug;

use crate::domain::model::task::{TaskValidationError};
use crate::usecase::task::TaskError;
use crate::interface::presenter::task::TaskMapper;
use openapi::apis::tasks::{Tasks, TasksCompletedGetResponse, TasksGetResponse, TasksIdCompletePutResponse, TasksIdDeleteResponse, TasksIdGetResponse, TasksIdPutResponse, TasksIdUncompletePutResponse, TasksPendingGetResponse, TasksPostResponse, TasksSearchGetResponse};
use openapi::models::{TasksIdCompletePutPathParams, TasksIdDeletePathParams, TasksIdGetPathParams, TasksIdPutPathParams, TasksIdUncompletePutPathParams, TasksSearchGetQueryParams};
use tracing;

/// API実装のエラー型
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<TaskError> for ApiError {
    fn from(error: TaskError) -> Self {
        match error {
            TaskError::NotFound(id) => ApiError::TaskNotFound(format!("Task with id {} not found", id)),
            TaskError::Validation(msg) => ApiError::ValidationError(msg.to_string()),
            TaskError::Repository(msg) => ApiError::InternalError(msg),
        }
    }
}

impl From<TaskValidationError> for ApiError {
    fn from(error: TaskValidationError) -> Self {
        ApiError::ValidationError(error.to_string())
    }
}

/// APIトレイトの実装
#[derive(Clone)]
pub struct TaskApiImpl<T> {
    pub usecase: T,
}

impl<T> TaskApiImpl<T> {
    pub fn new(task_usecase: T) -> Self {
        Self { usecase: task_usecase }
    }
}

impl<T> AsRef<TaskApiImpl<T>> for TaskApiImpl<T> {
    fn as_ref(&self) -> &TaskApiImpl<T> {
        self
    }
}

#[async_trait]
impl<T> Tasks<ApiError> for TaskApiImpl<T>
where
    T: crate::usecase::task::TaskUsecase + Send + Sync + 'static,
{
    async fn tasks_completed_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<TasksCompletedGetResponse, ApiError> {
        let domain_tasks = self.usecase.get_completed_tasks().await?;
        let api_tasks = TaskMapper::domain_vec_to_api(domain_tasks);
        Ok(TasksCompletedGetResponse::Status200_ListOfCompletedTasks(api_tasks))
    }

    async fn tasks_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<TasksGetResponse, ApiError> {
        let domain_tasks = self.usecase.get_all_tasks().await?;
        let api_tasks = TaskMapper::domain_vec_to_api(domain_tasks);
        Ok(TasksGetResponse::Status200_ListOfAllTasks(api_tasks))
    }

    async fn tasks_id_complete_put(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &TasksIdCompletePutPathParams,
    ) -> Result<TasksIdCompletePutResponse, ApiError> {
        let task_id = path_params.id as u64;
        let domain_task = self.usecase.complete_task(task_id).await?;
        let api_task = TaskMapper::domain_to_api(domain_task);
        Ok(TasksIdCompletePutResponse::Status200_TaskMarkedAsCompleted(api_task))
    }

    async fn tasks_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &TasksIdDeletePathParams,
    ) -> Result<TasksIdDeleteResponse, ApiError> {
        let task_id = path_params.id as u64;
        self.usecase.delete_task(task_id).await?;
        Ok(TasksIdDeleteResponse::Status204_TaskDeletedSuccessfully)
    }

    async fn tasks_id_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &TasksIdGetPathParams,
    ) -> Result<TasksIdGetResponse, ApiError> {
        let task_id = path_params.id as u64;
        let domain_task = self.usecase.get_task_by_id(task_id).await?;
        let api_task = TaskMapper::domain_to_api(domain_task);
        Ok(TasksIdGetResponse::Status200_TaskFound(api_task))
    }

    async fn tasks_id_put(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &TasksIdPutPathParams,
        body: &openapi::models::UpdateTask,
    ) -> Result<TasksIdPutResponse, ApiError> {
        let task_id = path_params.id as u64;
        let domain_update = TaskMapper::api_update_to_domain(body.clone())?;
        let domain_task = self.usecase.update_task(task_id, domain_update).await?;
        let api_task = TaskMapper::domain_to_api(domain_task);
        Ok(TasksIdPutResponse::Status200_TaskUpdatedSuccessfully(api_task))
    }

    async fn tasks_id_uncomplete_put(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &TasksIdUncompletePutPathParams,
    ) -> Result<TasksIdUncompletePutResponse, ApiError> {
        let task_id = path_params.id as u64;
        let domain_task = self.usecase.uncomplete_task(task_id).await?;
        let api_task = TaskMapper::domain_to_api(domain_task);
        Ok(TasksIdUncompletePutResponse::Status200_TaskMarkedAsUncompleted(api_task))
    }

    async fn tasks_pending_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<TasksPendingGetResponse, ApiError> {
        let domain_tasks = self.usecase.get_pending_tasks().await?;
        let api_tasks = TaskMapper::domain_vec_to_api(domain_tasks);
        Ok(TasksPendingGetResponse::Status200_ListOfPendingTasks(api_tasks))
    }

    async fn tasks_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        body: &openapi::models::CreateTask,
    ) -> Result<TasksPostResponse, ApiError> {
        let domain_create = TaskMapper::api_create_to_domain(body.clone())?;
        let domain_task = self.usecase.create_task(domain_create).await?;
        let api_task = TaskMapper::domain_to_api(domain_task);
        Ok(TasksPostResponse::Status201_TaskCreatedSuccessfully(api_task))
    }

    async fn tasks_search_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        query_params: &TasksSearchGetQueryParams,
    ) -> Result<TasksSearchGetResponse, ApiError> {
        let query = query_params.q.as_deref().unwrap_or("");
        let domain_tasks = self.usecase.search_tasks(query).await?;
        let api_tasks = TaskMapper::domain_vec_to_api(domain_tasks);
        Ok(TasksSearchGetResponse::Status200_SearchResults(api_tasks))
    }
}

// エラーハンドラーの実装
#[async_trait]
impl<T> openapi::apis::ErrorHandler<ApiError> for TaskApiImpl<T>
where
    T: crate::usecase::task::TaskUsecase + Send + Sync + 'static,
{
    async fn handle_error(
        &self,
        _method: &http::Method,
        _host: &axum_extra::extract::Host,
        _cookies: &axum_extra::extract::CookieJar,
        error: ApiError,
    ) -> Result<axum::response::Response, http::StatusCode> {
        tracing::error!("API Error: {:?}", error);
        match error {
            ApiError::TaskNotFound(_) => {
                Ok(axum::response::Response::builder()
                    .status(axum::http::StatusCode::NOT_FOUND)
                    .body(axum::body::Body::empty())
                    .unwrap())
            }
            ApiError::ValidationError(_) => {
                Ok(axum::response::Response::builder()
                    .status(axum::http::StatusCode::BAD_REQUEST)
                    .body(axum::body::Body::empty())
                    .unwrap())
            }
            ApiError::InternalError(_) => {
                Ok(axum::response::Response::builder()
                    .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(axum::body::Body::empty())
                    .unwrap())
            }
        }
    }
} 