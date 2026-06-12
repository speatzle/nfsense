pub mod api;
pub mod apply;
pub mod config_manager;
pub mod definitions;
pub mod state;
pub mod templates;
pub mod validation;
pub mod web;

#[macro_use]
extern crate lazy_static;

pub const CLIENT_INDEX_PATH: &str = "/usr/share/nfsense/html/index.html";
pub const CLIENT_FAVICON_PATH: &str = "/usr/share/nfsense/html/favicon.svg";
pub const CLIENT_ASSETS_PATH: &str = "/usr/share/nfsense/html/assets";
pub const HTTPS_CERT_PATH: &str = "/var/lib/nfsense/cert.pem";
pub const HTTPS_KEY_PATH: &str = "/var/lib/nfsense/key.pem";
