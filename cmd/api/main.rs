use todo_api::infrastructure::server;

#[tokio::main]
async fn main() {
    server::start_server().await;
}

