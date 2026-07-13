use super::util;
use super::ApplyError;
use crate::{
    definitions::{
        config::Config, network::NetworkInterfaceType, object::Address, object::AddressType,
        vpn::WireguardEndpoint,
    },
    templates,
};
use ipnet::IpNet;
use serde::Serialize;
use std::net::IpAddr;
use std::process::Command;
use std::{error::Error, io::Write};
use structdb_core::Database;
use tera::Context;
use tracing::{error, info};

const NETWORKD_CONFIG_PATH: &str = "/run/systemd/network";

#[derive(Serialize)]
struct PeerContext {
    public_key: String,
    preshared_key: Option<String>,
    allowed_ips: Vec<String>,
    endpoint: Option<String>,
    persistent_keepalive: Option<u64>,
}

pub struct File {
    pub name: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct StaticRouteContext {
    pub destination: IpNet,
    pub gateway: IpAddr,
    pub metric: u64,
}

pub fn delete_files_in_folder(path: &str) -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        std::fs::remove_file(entry.path())?;
    }
    Ok(())
}

pub fn create_files_in_folder(path: &str, files: Vec<File>) -> std::io::Result<()> {
    for file in files {
        let mut f = std::fs::File::create(path.to_string() + "/" + &file.name)?;
        f.write_all(file.content.as_bytes())?;
    }
    Ok(())
}

pub fn generate_networkd(
    apply: bool,
    pending_config: Config,
    current_config: Config,
) -> Result<(), ApplyError> {
    let files = generate_networkd_config_files(pending_config, current_config)?;

    info!("Deleting old Networkd Configs");
    match delete_files_in_folder(NETWORKD_CONFIG_PATH) {
        Ok(_) => (),
        Err(err) => return Err(ApplyError::IOError(err)),
    }

    info!("Writing new Networkd Configs");
    match create_files_in_folder(NETWORKD_CONFIG_PATH, files) {
        Ok(_) => (),
        Err(err) => return Err(ApplyError::IOError(err)),
    }

    if apply {
        info!("Restarting Networkd");
        match Command::new("systemctl")
            .arg("restart")
            .arg("systemd-networkd")
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

pub fn generate_networkd_config_files(
    pending_config: Config,
    _current_config: Config,
) -> Result<Vec<File>, ApplyError> {
    let mut files = Vec::new();

    // Step 0 Generate vrf netdev files
    for virtual_router in &pending_config.network.virtual_routers {
        let mut context = Context::new();
        context.insert("name", &virtual_router.name);
        context.insert("table_id", &virtual_router.table_id);

        files.push(generate_config_file(
            context,
            "networkd/create-vrf.netdev",
            format!("05-create-vrf-{}.netdev", &virtual_router.name),
        )?);
    }

    // Step 1 Generate vlan netdev files
    for interface in &pending_config.network.interfaces {
        if let NetworkInterfaceType::Vlan(vlan) = &interface.interface_type {
            let mut context = Context::new();
            context.insert("name", &interface.name);
            context.insert("vlan_id", &vlan.id);

            files.push(generate_config_file(
                context,
                "networkd/create-vlan.netdev",
                format!("10-create-vlan-{}.netdev", &interface.name),
            )?);
        }
    }

    // Step 2 Generate bond netdev files
    for interface in &pending_config.network.interfaces {
        if let NetworkInterfaceType::Bond(bond) = &interface.interface_type {
            let mut context = Context::new();
            context.insert("name", &interface.name);

            files.push(generate_config_file(
                context,
                "networkd/create-bond.netdev",
                format!("20-create-bond-{}.netdev", &interface.name),
            )?);

            // Create Membership files
            for member in bond.get_members(&pending_config) {
                let mut context = Context::new();
                context.insert("bond_name", &interface.name);

                // if interface is a hardware interface then we want to use device instead
                match &member.interface_type {
                    NetworkInterfaceType::Hardware { device } => context.insert("name", device),
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
        if let NetworkInterfaceType::Bridge(bridge) = &interface.interface_type {
            let mut context = Context::new();
            context.insert("name", &interface.name);

            files.push(generate_config_file(
                context,
                "networkd/create-bridge.netdev",
                format!("30-create-bridge-{}.netdev", &interface.name),
            )?);

            // Create Membership files
            for member in bridge.get_members(&pending_config) {
                let mut context = Context::new();
                context.insert("bridge_name", &interface.name);

                // if interface is a hardware interface then we want to use device instead
                match &member.interface_type {
                    NetworkInterfaceType::Hardware { device } => context.insert("name", device),
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
        let mut peers = Vec::new();
        for peer in interface.get_peers(&pending_config) {
            let allowed_ips = util::convert_addresses_to_strings(
                peer.get_allowed_ips(&pending_config).cloned().collect(),
            );

            // Resolve endpoint address + port to formatted string
            let endpoint = match &peer.endpoint {
                WireguardEndpoint::Specify { address, port } => {
                    let addr = pending_config
                        .get::<Address>(address)
                        .expect("validation ensures endpoint address exists");
                    match &addr.address_type {
                        AddressType::Host { address } => Some(format!("{}:{}", address, port)),
                        _ => unreachable!("validation enforces Host type for endpoint"),
                    }
                }
                WireguardEndpoint::None => None,
            };

            peers.push(PeerContext {
                public_key: peer.public_key.clone(),
                preshared_key: peer.preshared_key.clone(),
                allowed_ips,
                endpoint,
                persistent_keepalive: peer.persistent_keepalive,
            });
        }

        context.insert("peers", &peers);

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
                NetworkInterfaceType::Vlan(vlan_details) => {
                    if vlan_details.parent == interface.name {
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
                let gateway_address =
                    static_route.get_gateway(&pending_config).ok_or_else(|| {
                        ApplyError::ObjectReferenceError(format!(
                            "Static route '{}' references non-existent gateway address",
                            static_route.name
                        ))
                    })?;
                match gateway_address.address_type {
                    AddressType::Host { address } => {
                        static_routes.push(StaticRouteContext {
                            destination: static_route.destination,
                            gateway: address,
                            metric: static_route.metric,
                        });
                    }
                    _ => {
                        return Err(ApplyError::ObjectReferenceError(format!(
                            "Static route '{}' gateway must be a Host address, got {:?}",
                            static_route.name, gateway_address.address_type
                        )));
                    }
                };
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
