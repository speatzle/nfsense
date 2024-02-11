use thiserror::Error;

pub mod chrony;
pub mod networkd;

#[derive(Error, Debug)]
pub enum ApplyError {
    #[error("Template Error")]
    TemplateError(#[from] tera::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AddrParseError(#[from] ipnet::AddrParseError),

    #[error("Service Restart Failed")]
    ServiceRestartFailed,
}
