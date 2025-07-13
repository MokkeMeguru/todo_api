use super::http::generated_routes::create_generated_router;

pub async fn start_server() {
    // 生成されたルーターを使用
    let app = create_generated_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
} 