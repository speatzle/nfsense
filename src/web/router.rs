use axum::{middleware, Router};

use tower_cookies::CookieManagerLayer;

#[derive(Clone)]
struct tmp {}

pub async fn get_router() -> Router {
    Router::new()
        .merge(super::auth::routes())
        .nest("/api", super::rpc::routes())
        .layer(middleware::from_fn_with_state(tmp {}, super::auth::mw_auth))
        .layer(CookieManagerLayer::new())
    // .fallback_service(service)
}
