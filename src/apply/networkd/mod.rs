use super::ApplyError;
use crate::{
    definitions::{config::Config, network::NetworkInterfaceType},
    templates,
};
use std::error::Error;
use tera::Context;
use tracing::{error, info};

pub struct File {
    pub name: String,
    pub content: String,
}

pub fn apply_networkd(pending_config: Config, current_config: Config) -> Result<(), ApplyError> {
    let files = generate_networkd_config_files(pending_config, current_config)?;
    info!("Got Files");
    for file in files {
        info!("Conf File {}", file.name);
        info!("\n{}", file.content);
    }
    Ok(())
}

pub fn generate_networkd_config_files(
    pending_config: Config,
    _current_config: Config,
) -> Result<Vec<File>, ApplyError> {
    let mut files = Vec::new();

    // Step 1 Generate vlan netdev files
    for interface in &pending_config.network.interfaces {
        if let NetworkInterfaceType::Vlan { id, .. } = &interface.interface_type {
            let mut context = Context::new();
            context.insert("name", &interface.name);
            context.insert("vlan_id", &id);

            files.push(generate_config_file(
                context,
                "networkd/create-vlan.netdev",
                format!("10-create-vlan-{}.netdev", &interface.name),
            )?);
        }
    }

    // Step 2 Generate bond netdev files
    for interface in &pending_config.network.interfaces {
        if let NetworkInterfaceType::Bond { .. } = &interface.interface_type {
            let mut context = Context::new();
            context.insert("name", &interface.name);

            files.push(generate_config_file(
                context,
                "networkd/create-bond.netdev",
                format!("20-create-bond-{}.netdev", &interface.name),
            )?);

            // Create Membership files
            for member in interface
                .interface_type
                .bond_members(pending_config.clone())
            {
                let mut context = Context::new();
                context.insert("bond_name", &interface.name);

                // if interface is a hardware interface then we want to use device instead
                match member.interface_type {
                    NetworkInterfaceType::Hardware { device } => context.insert("name", &device),
                    _ => context.insert("name", &member.name),
                };

                files.push(generate_config_file(
                    context,
                    "networkd/bond-membership.network",
                    format!("50-bond-membership-{}.network", &member.name),
                )?);
            }
        }
    }

    // Step 3 Generate bridge netdev files
    for interface in &pending_config.network.interfaces {
        if let NetworkInterfaceType::Bridge { .. } = &interface.interface_type {
            let mut context = Context::new();
            context.insert("name", &interface.name);

            files.push(generate_config_file(
                context,
                "networkd/create-bridge.netdev",
                format!("30-create-bridge-{}.netdev", &interface.name),
            )?);

            // Create Membership files
            for member in interface
                .interface_type
                .bridge_members(pending_config.clone())
            {
                let mut context = Context::new();
                context.insert("bridge_name", &interface.name);

                // if interface is a hardware interface then we want to use device instead
                match member.interface_type {
                    NetworkInterfaceType::Hardware { device } => context.insert("name", &device),
                    _ => context.insert("name", &member.name),
                };

                files.push(generate_config_file(
                    context,
                    "networkd/bridge-membership.network",
                    format!("60-bridge-membership-{}.network", &member.name),
                )?);
            }
        }
    }

    // Step 4 Generate wireguard netdev files
    for interface in &pending_config.vpn.wireguard.interfaces {
        let mut context = Context::new();
        context.insert("interface", &interface);
        context.insert("peers", &interface.peers(pending_config.clone()));

        files.push(generate_config_file(
            context,
            "networkd/create-wireguard.netdev",
            format!("40-create-wireguard-{}.netdev", &interface.name),
        )?);
    }

    // Step 5 Generate Addressing network files
    for interface in &pending_config.network.interfaces {
        let mut context = Context::new();
        match &interface.interface_type {
            NetworkInterfaceType::Hardware { device } => context.insert("name", &device),
            _ => context.insert("name", &interface.name),
        };

        context.insert("interface", &interface);

        // List of all vlans that have this interface as a parent
        let mut vlans = Vec::new();
        // TODO Use Backreferenceing instead of loop and if
        for vlan in &pending_config.network.interfaces {
            match &vlan.interface_type {
                NetworkInterfaceType::Vlan { parent, .. } => {
                    if parent == &interface.name {
                        vlans.push(vlan.name.clone());
                    }
                }
                _ => (),
            };
        }
        context.insert("vlans", &vlans);

        // List all Static Routes for this interface
        let mut static_routes = Vec::new();
        // TODO Use Backreferenceing instead of loop and if
        for static_route in &pending_config.network.static_routes {
            if static_route.interface == interface.name {
                static_routes.push(static_route);
            }
        }
        context.insert("static_routes", &static_routes);

        files.push(generate_config_file(
            context,
            "networkd/config-addressing.network",
            format!("70-config-addressing-{}.network", &interface.name),
        )?);
    }

    Ok(files)
}

fn generate_config_file(
    context: Context,
    template_name: &str,
    file_name: String,
) -> Result<File, ApplyError> {
    match templates::TEMPLATES.render(template_name, &context) {
        Ok(s) => Ok(File {
            name: file_name,
            content: s,
        }),
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
}
