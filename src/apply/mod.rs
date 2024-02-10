use thiserror::Error;

pub mod networkd;

#[derive(Error, Debug)]
pub enum ApplyError {
    #[error("Template Error")]
    TemplateError(#[from] tera::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Service Restart Failed")]
    ServiceRestartFailed,
}
