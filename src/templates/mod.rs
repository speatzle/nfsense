use tera::Tera;

// TODO, better way to handle this?
#[cfg(not(debug_assertions))]
pub const TEMPLATE_PATH: &str = "/usr/share/nfsense/templates/**/*.*";
#[cfg(debug_assertions)]
pub const TEMPLATE_PATH: &str = "src/templates/**/*.*";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new(TEMPLATE_PATH) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
