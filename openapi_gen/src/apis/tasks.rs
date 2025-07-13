use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksCompletedGetResponse {
    /// List of completed tasks
    Status200_ListOfCompletedTasks
    (Vec<models::Task>)
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksGetResponse {
    /// List of all tasks
    Status200_ListOfAllTasks
    (Vec<models::Task>)
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksIdCompletePutResponse {
    /// Task marked as completed
    Status200_TaskMarkedAsCompleted
    (models::Task)
    ,
    /// Task not found
    Status404_TaskNotFound
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksIdDeleteResponse {
    /// Task deleted successfully
    Status204_TaskDeletedSuccessfully
    ,
    /// Task not found
    Status404_TaskNotFound
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksIdGetResponse {
    /// Task found
    Status200_TaskFound
    (models::Task)
    ,
    /// Task not found
    Status404_TaskNotFound
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksIdPutResponse {
    /// Task updated successfully
    Status200_TaskUpdatedSuccessfully
    (models::Task)
    ,
    /// Validation error or invalid operation
    Status400_ValidationErrorOrInvalidOperation
    ,
    /// Task not found
    Status404_TaskNotFound
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksIdUncompletePutResponse {
    /// Task marked as uncompleted
    Status200_TaskMarkedAsUncompleted
    (models::Task)
    ,
    /// Task not found
    Status404_TaskNotFound
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksPendingGetResponse {
    /// List of pending tasks
    Status200_ListOfPendingTasks
    (Vec<models::Task>)
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksPostResponse {
    /// Task created successfully
    Status201_TaskCreatedSuccessfully
    (models::Task)
    ,
    /// Validation error
    Status400_ValidationError
    ,
    /// Internal server error
    Status500_InternalServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksSearchGetResponse {
    /// Search results
    Status200_SearchResults
    (Vec<models::Task>)
    ,
    /// Internal server error
    Status500_InternalServerError
}


/// Tasks
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Tasks<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Get all completed tasks.
    ///
    /// TasksCompletedGet - GET /tasks/completed
    async fn tasks_completed_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<TasksCompletedGetResponse, E>;

    /// Get all tasks.
    ///
    /// TasksGet - GET /tasks
    async fn tasks_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<TasksGetResponse, E>;

    /// Mark a task as completed.
    ///
    /// TasksIdCompletePut - PUT /tasks/{id}/complete
    async fn tasks_id_complete_put(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::TasksIdCompletePutPathParams,
    ) -> Result<TasksIdCompletePutResponse, E>;

    /// Delete a task.
    ///
    /// TasksIdDelete - DELETE /tasks/{id}
    async fn tasks_id_delete(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::TasksIdDeletePathParams,
    ) -> Result<TasksIdDeleteResponse, E>;

    /// Get a task by ID.
    ///
    /// TasksIdGet - GET /tasks/{id}
    async fn tasks_id_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::TasksIdGetPathParams,
    ) -> Result<TasksIdGetResponse, E>;

    /// Update a task.
    ///
    /// TasksIdPut - PUT /tasks/{id}
    async fn tasks_id_put(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::TasksIdPutPathParams,
            body: &models::UpdateTask,
    ) -> Result<TasksIdPutResponse, E>;

    /// Mark a task as uncompleted.
    ///
    /// TasksIdUncompletePut - PUT /tasks/{id}/uncomplete
    async fn tasks_id_uncomplete_put(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::TasksIdUncompletePutPathParams,
    ) -> Result<TasksIdUncompletePutResponse, E>;

    /// Get all pending tasks.
    ///
    /// TasksPendingGet - GET /tasks/pending
    async fn tasks_pending_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<TasksPendingGetResponse, E>;

    /// Create a new task.
    ///
    /// TasksPost - POST /tasks
    async fn tasks_post(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::CreateTask,
    ) -> Result<TasksPostResponse, E>;

    /// Search tasks by description.
    ///
    /// TasksSearchGet - GET /tasks/search
    async fn tasks_search_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::TasksSearchGetQueryParams,
    ) -> Result<TasksSearchGetResponse, E>;
}
