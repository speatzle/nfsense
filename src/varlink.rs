use serde::{Deserialize, Serialize};
use std::{os::unix::fs::PermissionsExt, thread};
use tokio::fs;
use tracing::error;
use zlink::{introspect, proxy, service, unix, ReplyError, Server};

#[derive(Debug, Serialize, Deserialize, introspect::Type)]
pub struct Pong {
    pub message: String,
}

#[derive(Debug, Clone, ReplyError, introspect::ReplyError)]
#[zlink(interface = "net.nfsense")]
pub enum NfsenseError {
    InternalError { message: String },
}

#[proxy("net.nfsense")]
pub trait NfsenseProxy {
    async fn ping(&mut self) -> zlink::Result<Result<Pong, NfsenseError>>;
}

pub struct NfsenseVarlink;

#[service(interface = "net.nfsense")]
impl NfsenseVarlink {
    async fn ping(&self) -> Pong {
        Pong {
            message: "pong".to_string(),
        }
    }
}

const VARLINK_SOCKET_PATH: &str = "/run/nfsense/net.nfsense";

pub fn start() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = std::fs::remove_file(VARLINK_SOCKET_PATH);
    let listener = unix::bind(VARLINK_SOCKET_PATH)?;
    let _ = fs::set_permissions(VARLINK_SOCKET_PATH, std::fs::Permissions::from_mode(0o600));

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("failed to create varlink runtime");
        rt.block_on(async move {
            let server = Server::new(listener, NfsenseVarlink);
            if let Err(e) = server.run().await {
                error!("Varlink server error: {e}");
            }
        });
    });

    Ok(())
}
