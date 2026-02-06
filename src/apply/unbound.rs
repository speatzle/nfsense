use super::ApplyError;
use crate::{
    definitions::{
        config::Config,
        network::{AddressingMode, NetworkInterfaceType},
    },
    templates,
};
use std::process::Command;
use std::{error::Error, io::Write};
use tera::Context;
use tracing::{error, info};

const UNBOUND_CONFIG_PATH: &str = "/run/nfsense/unbound/unbound.conf";
const UNBOUND_TEMPLATE_PATH: &str = "unbound/unbound.conf";

pub fn generate_unbound(
    apply: bool,
    pending_config: Config,
    _current_config: Config,
) -> Result<(), ApplyError> {
    let config_data;
    let mut context = Context::new();
    let mut interfaces = vec![];
    let mut subnets = vec![];

    for server in &pending_config.service.dns_servers {
        let interface = server.interface(pending_config.clone());
        if let NetworkInterfaceType::Hardware { device } = interface.interface_type.clone() {
            interfaces.push(device);
        } else {
            interfaces.push(interface.name.clone());
        }

        if let AddressingMode::Static { address } =
            &server.interface(pending_config.clone()).addressing_mode
        {
            subnets.push(address.trunc().to_string());
        }
    }
    context.insert("interfaces", &interfaces);
    context.insert("subnets", &subnets);

    match templates::TEMPLATES.render(UNBOUND_TEMPLATE_PATH, &context) {
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

    info!("Deleting old Unbound Config");
    std::fs::remove_file(UNBOUND_CONFIG_PATH)?;

    info!("Writing new Unbound Config");
    let mut f = std::fs::File::create(UNBOUND_CONFIG_PATH)?;
    f.write_all(config_data.as_bytes())?;

    if apply {
        info!("Restarting Unbound");
        match Command::new("systemctl")
            .arg("restart")
            .arg("unbound")
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
    } else {
        Ok(())
    }
}
