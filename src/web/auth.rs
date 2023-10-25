use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use super::super::AppState;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;

use axum::{
    extract::Extension,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

const SESSION_COOKIE: &str = "session";

#[derive(Clone, Debug)]
pub struct SessionState {
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
}

#[derive(Clone, Debug)]
pub struct Session {
    username: String,
    //expires: time,
}

#[derive(Clone, Debug, Deserialize)]
struct LoginParameters {
    username: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/session", post(session_handler))
}

async fn login_handler(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(payload): Json<LoginParameters>,
    // mut session_state: SessionState,
) -> impl IntoResponse {
    //cookies.add(Cookie::new());
    todo!()
}

async fn logout_handler(cookies: Cookies, app_state: State<AppState>) -> impl IntoResponse {
    /*
    if let Some(session_cookie) = cookies.get(SESSION_COOKIE) {
        let session_id = session_cookie.value();

        // TODO check that sessions_id is a valid uuidv4

        session_state.sessions.remove(session_id);
        cookies.remove(session_cookie);
    }
    */
    todo!()
}

async fn session_handler(cookies: Cookies, State(state): State<AppState>) -> impl IntoResponse {
    //return Err(StatusCode::UNAUTHORIZED);
    todo!()
}

pub async fn mw_auth<B>(
    app_state: State<AppState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
    // session_state: SessionState,
) -> Result<Response, StatusCode> {
    /*
    if let Some(session_cookie) = cookies.get(SESSION_COOKIE) {
        let session_id = session_cookie.value();

        // TODO check that sessions_id is a valid uuidv4

        if let Some(session) = session_state.sessions.get(session_id) {
            req.extensions_mut().insert(session.clone());
            return Ok(next.run(req).await);
        }
    }
    return Err(StatusCode::UNAUTHORIZED);
    */
    todo!()
}
