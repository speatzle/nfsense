use super::super::AppState;
use axum::routing::post;
use axum::{Json, Router};
use tower_cookies::{Cookie, Cookies};

use axum::{
    extract::Extension,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

use tracing::info;

pub fn routes() -> Router<super::super::AppState> {
    Router::new().route("/api", post(api_handler))
}

async fn api_handler(
    State(state): State<AppState>,
    session: Extension<super::auth::Session>,
) -> impl IntoResponse {
    info!("api hit! user: {:?}", session.username);
    StatusCode::NOT_IMPLEMENTED
}
