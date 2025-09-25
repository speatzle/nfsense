use thiserror::Error;

pub mod chrony;
pub mod kea;
pub mod networkd;
pub mod nftables;
pub mod unbound;

#[derive(Error, Debug)]
pub enum ApplyError {
    #[error("Template Error")]
    TemplateError(#[from] tera::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AddrParseError(#[from] ipnet::AddrParseError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("Service Restart Failed")]
    ServiceRestartFailed,

    #[error("Config Apply Failed")]
    ConfigApplyFailed,

    #[error("Config Check Failed")]
    ConfigCheckFailed,
}
