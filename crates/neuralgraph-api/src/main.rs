use axum::{
    routing::get,
    Json,
    Router,
};
use serde_json::json;
use tokio::net::TcpListener;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "NeuralGraph API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("NeuralGraph API running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start API server");
}

