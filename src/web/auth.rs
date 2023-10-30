use std::collections::HashMap;

use std::sync::{Arc, RwLock};
use uuid::Uuid;

use super::super::AppState;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use pwhash::sha512_crypt;
use tracing::info;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("No Session Cookie Found")]
    NoSessionCookie,

    #[error("Invalid Session")]
    InvalidSession,
}

const SESSION_COOKIE: &str = "session";

#[derive(Clone, Debug)]
pub struct SessionState {
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub username: String,
    //expires: time,
    // TODO have permissions here for fast access, update Permissions with a config manager apply function
    // permissions
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
) -> impl IntoResponse {
    if let Some(user) = state
        .config_manager
        .get_current_config()
        .system
        .users
        .get(&payload.username.to_string())
    {
        if sha512_crypt::verify(payload.password, &user.hash) {
            let mut sessions = state.session_state.sessions.write().unwrap();
            let id = Uuid::new_v4().to_string();

            sessions.insert(
                id.clone(),
                Session {
                    username: payload.username.clone(),
                },
            );

            cookies.add(Cookie::new(SESSION_COOKIE, id));
            info!("user logged in: {:?}", payload.username);
            return StatusCode::OK;
        }
    }

    info!("user login failed: {:?}", payload.username);
    StatusCode::UNAUTHORIZED
}

async fn logout_handler(cookies: Cookies, state: State<AppState>) -> impl IntoResponse {
    let session_cookie = cookies.get(SESSION_COOKIE);
    match session_cookie {
        Some(s) => {
            let session_id = s.value();

            let mut sessions = state.session_state.sessions.write().unwrap();

            // TODO Fix Cookie remove
            // cookies.remove(s.clone());

            if let Some(session) = sessions.get(session_id) {
                info!("user logged out: {:?}", session.username);
                sessions.remove(session_id);
                return StatusCode::OK;
            }
            return StatusCode::UNAUTHORIZED;
        }
        None => return StatusCode::UNAUTHORIZED,
    }
}

fn get_session(cookies: Cookies, state: SessionState) -> Result<Session, AuthError> {
    let session_cookie = cookies.get(SESSION_COOKIE);
    match session_cookie {
        Some(s) => {
            let session_id = s.value();

            let sessions = state.sessions.write().unwrap();

            if let Some(session) = sessions.get(session_id) {
                return Ok(session.clone());
            }
            return Err(AuthError::InvalidSession);
        }
        None => return Err(AuthError::NoSessionCookie),
    }
}

async fn session_handler(cookies: Cookies, State(state): State<AppState>) -> impl IntoResponse {
    match get_session(cookies, state.session_state) {
        // TODO Return build git commit hash as json result for frontend reloading
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::UNAUTHORIZED,
    }
}

pub async fn mw_auth<B>(
    state: State<AppState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
    // session_state: SessionState,
) -> Result<Response, StatusCode> {
    match get_session(cookies, state.session_state.clone()) {
        Ok(session) => {
            req.extensions_mut().insert(session.clone());
            return Ok(next.run(req).await);
        }
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
}
