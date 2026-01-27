use super::ApplyError;
use crate::definitions::{
    config::Config,
    network::{AddressingMode, NetworkInterfaceType},
    object::AddressType,
    service::{DNSServerMode, GatewayMode, NTPServerMode},
};
use ipnet::IpNet;
use serde::Serialize;
use std::io::Write;
use std::process::Command;
use tracing::info;

const KEA_V4_CONFIG_PATH: &str = "/etc/kea/kea-dhcp4.conf";
// const KEA_V6_CONFIG_PATH: &str = "/etc/kea/kea-dhcp6.conf";

#[derive(Serialize, Clone, Debug)]
pub struct KeaConfig {
    #[serde(rename = "Dhcp4")]
    pub dhcpv4: KeaDHCPv4,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeaDHCPv4 {
    #[serde(rename = "valid-lifetime")]
    pub valid_lifetime: u64,
    #[serde(rename = "renew-timer")]
    pub renew_timer: u64,
    #[serde(rename = "rebind-timer")]
    pub rebind_timer: u64,

    #[serde(rename = "interfaces-config")]
    pub interfaces_config: KeaInterfaces,

    #[serde(rename = "lease-database")]
    pub lease_database: KeaLeases,

    #[serde(rename = "subnet4")]
    pub subnet4: Vec<KeaSubnet4>,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeaInterfaces {
    #[serde(rename = "interfaces")]
    pub interfaces: Vec<String>,
    #[serde(rename = "dhcp-socket-type")]
    pub dhcp_socket_type: String,
    #[serde(rename = "service-sockets-require-all")]
    pub service_sockets_require_all: bool,
    #[serde(rename = "service-sockets-max-retries")]
    pub service_sockets_max_retries: u64,
    #[serde(rename = "service-sockets-retry-wait-time")]
    pub service_sockets_retry_wait_time: u64,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeaLeases {
    #[serde(rename = "type")]
    pub database_type: String,
    #[serde(rename = "persist")]
    pub persist: bool,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeaSubnet4 {
    // TODO add subnet Id https://kea.readthedocs.io/en/kea-2.2.0/arm/dhcp4-srv.html#ipv4-subnet-identifier
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(rename = "subnet")]
    pub subnet: IpNet,
    #[serde(rename = "pools")]
    pub pools: Vec<KeaPool>,
    #[serde(rename = "valid-lifetime")]
    pub valid_lifetime: u64,
    #[serde(rename = "option-data")]
    pub option_data: Vec<KeaOption>,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeaPool {
    #[serde(rename = "pool")]
    pub pool: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeaOption {
    #[serde(rename = "code")]
    pub code: u64,
    #[serde(rename = "data")]
    pub data: String,
}

pub fn apply_kea(pending_config: Config, _current_config: Config) -> Result<(), ApplyError> {
    let mut conf: KeaConfig = KeaConfig {
        dhcpv4: KeaDHCPv4 {
            valid_lifetime: 4000,
            renew_timer: 1000,
            rebind_timer: 2000,
            interfaces_config: KeaInterfaces {
                interfaces: vec![],
                dhcp_socket_type: "raw".to_owned(),
                service_sockets_require_all: true,
                service_sockets_max_retries: 3,
                service_sockets_retry_wait_time: 5000,
            },
            lease_database: KeaLeases {
                database_type: "memfile".to_string(),
                persist: true,
                name: "/var/lib/kea/kea-leases4.csv".to_string(),
            },
            subnet4: vec![],
        },
    };

    let mut dhcpindex = 0;
    for dhcp_server in pending_config.service.dhcp_servers.clone() {
        //TODO find a stable way for the subnet id
        dhcpindex += 1;
        let interface = dhcp_server.interface(pending_config.clone());
        // TODO specify main ip of interface https://kea.readthedocs.io/en/kea-2.2.0/arm/dhcp4-srv.html#interface-configuration
        match interface.interface_type {
            NetworkInterfaceType::Hardware { device } => {
                conf.dhcpv4.interfaces_config.interfaces.push(device)
            }
            _ => conf
                .dhcpv4
                .interfaces_config
                .interfaces
                .push(interface.name),
        }

        let mut subnet = KeaSubnet4 {
            id: dhcpindex,
            subnet: match interface.addressing_mode {
                AddressingMode::Static { address } => address.clone(),
                _ => panic!("Unsupported Addressing mode"),
            },
            pools: vec![],
            valid_lifetime: dhcp_server.lease_time,
            option_data: vec![],
        };

        match dhcp_server.gateway_mode.clone() {
            GatewayMode::Interface => subnet.option_data.push(KeaOption {
                code: 3,
                data: match interface.addressing_mode {
                    AddressingMode::Static { address } => address.addr().to_string(),
                    _ => panic!("Unsupported Address Type"),
                },
            }),
            GatewayMode::Specify { .. } => subnet.option_data.push(KeaOption {
                code: 3,
                data: match dhcp_server
                    .gateway_mode
                    .gateway(pending_config.clone())
                    .address_type
                {
                    AddressType::Host { address } => address.to_string(),
                    _ => panic!("Unsupported Address Type"),
                },
            }),
            GatewayMode::None => (),
        }

        match dhcp_server.dns_server_mode.clone() {
            DNSServerMode::Interface => subnet.option_data.push(KeaOption {
                code: 6,
                data: match interface.addressing_mode {
                    AddressingMode::Static { address } => address.addr().to_string(),
                    _ => panic!("Unsupported Address Type"),
                },
            }),
            DNSServerMode::Specify { .. } => {
                let mut servers = "".to_string();
                let dns_servers = dhcp_server
                    .dns_server_mode
                    .dns_servers(pending_config.clone());

                for i in 0..dns_servers.len() {
                    match dns_servers[i].address_type {
                        AddressType::Host { address } => {
                            if i > 0 {
                                servers += ", ";
                            }
                            servers += &address.to_string();
                        }
                        _ => panic!("Unsupported Address Type"),
                    }
                }

                subnet.option_data.push(KeaOption {
                    code: 6,
                    data: servers,
                });
            }
            DNSServerMode::None => (),
        }

        match dhcp_server.ntp_server_mode.clone() {
            NTPServerMode::Interface => subnet.option_data.push(KeaOption {
                code: 42,
                data: match interface.addressing_mode {
                    AddressingMode::Static { address } => address.addr().to_string(),
                    _ => panic!("Unsupported Address Type"),
                },
            }),
            NTPServerMode::Specify { .. } => {
                let mut servers = "".to_string();
                let ntp_servers = dhcp_server
                    .ntp_server_mode
                    .ntp_servers(pending_config.clone());

                for i in 0..ntp_servers.len() {
                    match ntp_servers[i].address_type {
                        AddressType::Host { address } => {
                            if i > 0 {
                                servers += ", ";
                            }
                            servers += &address.to_string();
                        }
                        _ => panic!("Unsupported Address Type"),
                    }
                }

                subnet.option_data.push(KeaOption {
                    code: 42,
                    data: servers,
                });
            }
            NTPServerMode::None => (),
        }

        let pools = dhcp_server.pool(pending_config.clone());
        for pool in pools {
            match pool.address_type {
                AddressType::Host { address } => subnet.pools.push(KeaPool {
                    pool: address.to_string() + "/32",
                }),
                AddressType::Range { .. } => panic!("TODO fix range type"),
                AddressType::Network { network } => subnet.pools.push(KeaPool {
                    pool: network.to_string(),
                }),
                AddressType::Group { .. } => panic!("TODO"),
            }
        }

        conf.dhcpv4.subnet4.push(subnet);
    }

    info!("Serializeing Kea v4 Config");
    let config_data: String = serde_json::to_string_pretty(&conf)?;

    info!("Deleting old Kea v4 Config");
    std::fs::remove_file(KEA_V4_CONFIG_PATH)?;

    info!("Writing new Kea v4 Config");
    let mut f = std::fs::File::create(KEA_V4_CONFIG_PATH)?;
    f.write_all(config_data.as_bytes())?;

    info!("Restarting Kea");
    match Command::new("systemctl")
        .arg("restart")
        .arg("kea-dhcp4-server")
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
