use crate::infrastructure::http::api_impl::TaskApiImpl;
use openapi::server::new as create_generated_server;
use crate::interface::gateway::inmemory::InMemoryTaskRepository;
use crate::usecase::task::TaskUsecaseImpl;

/// 生成されたサーバーを使用するルーターを作成
pub fn create_generated_router() -> axum::Router {
    let repository = InMemoryTaskRepository::new();
    let task_usecase = TaskUsecaseImpl::new(repository);
    let api_impl = TaskApiImpl::new(task_usecase);
    create_generated_server(api_impl)
} 