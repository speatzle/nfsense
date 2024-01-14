use thiserror::Error;

pub mod networkd;

#[derive(Error, Debug)]
pub enum ApplyError {
    #[error("Template Error")]
    TemplateError(#[from] tera::Error),
}
