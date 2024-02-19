use super::ApplyError;
use crate::{
    definitions::{config::Config, network::AddressingMode},
    templates,
};
use std::process::Command;
use std::{error::Error, io::Write};
use tera::Context;
use tracing::{error, info};

const CHRONY_CONFIG_PATH: &str = "/etc/chrony.conf";
const CHRONY_TEMPLATE_PATH: &str = "chrony/chrony.conf";

pub fn apply_chrony(pending_config: Config, _current_config: Config) -> Result<(), ApplyError> {
    let config_data;
    let mut context = Context::new();
    let mut subnets = vec![];

    for server in &pending_config.service.ntp_servers {
        if let AddressingMode::Static { address } =
            &server.interface(pending_config.clone()).addressing_mode
        {
            subnets.push(address.network().to_string());
        }
    }
    context.insert("subnets", &subnets);

    match templates::TEMPLATES.render(CHRONY_TEMPLATE_PATH, &context) {
        Ok(s) => config_data = s,
        Err(e) => {
            error!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                error!("Reason: {}", e);
                cause = e.source();
            }
            return Err(ApplyError::TemplateError(e));
        }
    }

    info!("Deleting old Chrony Config");
    std::fs::remove_file(CHRONY_CONFIG_PATH)?;

    info!("Writing new Chrony Config");
    let mut f = std::fs::File::create(CHRONY_CONFIG_PATH)?;
    f.write_all(config_data.as_bytes())?;

    info!("Restarting Chrony");
    match Command::new("systemctl")
        .arg("restart")
        .arg("chronyd")
        .output()
    {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(ApplyError::ServiceRestartFailed)
            }
        }
        Err(err) => Err(ApplyError::IOError(err)),
    }
}
