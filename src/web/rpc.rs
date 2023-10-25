use axum::Router;

pub fn routes() -> Router<super::super::AppState> {
    Router::new()
}
