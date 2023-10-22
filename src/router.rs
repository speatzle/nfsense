use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use tracing::info;

pub async fn get_router() -> Router {
    Router::new().route("/health", get(health_checker_handler))
}

pub async fn health_checker_handler() -> impl IntoResponse {
    info!("health hit");
    const MESSAGE: &str = "Hello there";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
