use super::ApplyError;
use crate::definitions::firewall::{SNATType, Verdict};
use crate::definitions::object::{Address, AddressType, PortDefinition, Service, ServiceType};
use crate::{definitions::config::Config, templates};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::process::Command;
use std::{error::Error, io::Write};
use tera::Context;
use tracing::{error, info};

const NFTABLES_CONFIG_PATH: &str = "/etc/nfsense/nfsense-nftables.conf";
const NFTABLES_TEMPLATE_PATH: &str = "nftables/nftables.conf";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rule {
    pub name: String,
    pub services: Vec<String>,
    pub addresses: String,
    pub counter: bool,
    pub log: bool,
    pub verdict: Option<String>,
    pub destination_nat_action: Option<String>,
    pub source_nat_action: Option<String>,
}

fn convert_addresses_to_strings(addresses: Vec<Address>) -> Vec<String> {
    let mut list = vec![];
    for address in addresses {
        match address.address_type {
            AddressType::Host { address } => list.push(address.to_string()),
            AddressType::Range { range } => list.push(range.to_string()),
            AddressType::Network { network } => list.push(network.to_string()),
            AddressType::Group { .. } => {
                //TODO
            }
        }
    }
    list
}

fn convert_list_to_set(list: Vec<String>) -> String {
    if list.len() == 0 {
        return "".to_string();
    } else if list.len() == 1 {
        return list[0].clone();
    }

    let mut res = "{ ".to_string();

    for (index, element) in list.iter().enumerate() {
        res += element;
        if index < list.len() - 1 {
            res += ", ";
        }
    }
    res += " }";
    res
}

fn generate_address_matcher(
    source_addresses: Vec<Address>,
    negate_source: bool,
    destination_addresses: Vec<Address>,
    negate_destination: bool,
) -> Result<String, ApplyError> {
    let source_list = convert_addresses_to_strings(source_addresses);
    let destination_list = convert_addresses_to_strings(destination_addresses);
    let mut res = "".to_string();

    if source_list.len() > 0 {
        res += "ip saddr ";
        if negate_source {
            res += "!= ";
        }
        res += &convert_list_to_set(source_list);
        res += " ";
    }

    if destination_list.len() > 0 {
        res += "ip daddr ";
        if negate_destination {
            res += "!= ";
        }
        res += &convert_list_to_set(destination_list);
    }

    Ok(res)
}

fn generate_port_matcher(
    protocol: &str,
    source: PortDefinition,
    destination: PortDefinition,
) -> String {
    let source_string = match source {
        PortDefinition::Any => "".to_string(),
        PortDefinition::Single { port } => {
            protocol.to_string() + &" sport ".to_string() + &port.to_string()
        }
        PortDefinition::Range {
            start_port,
            end_port,
        } => {
            protocol.to_string()
                + &" sport ".to_string()
                + &start_port.to_string()
                + " - "
                + &end_port.to_string()
        }
    };

    let destination_string = match destination {
        PortDefinition::Any => "".to_string(),
        PortDefinition::Single { port } => {
            protocol.to_string() + &" dport ".to_string() + &port.to_string()
        }
        PortDefinition::Range {
            start_port,
            end_port,
        } => {
            protocol.to_string()
                + &" dport ".to_string()
                + &start_port.to_string()
                + " - "
                + &end_port.to_string()
        }
    };
    if source_string.len() != 0 && destination_string.len() != 0 {
        source_string + " " + &destination_string
    } else {
        source_string + &destination_string
    }
}

fn generate_service_matchers(services: Vec<Service>) -> Result<Vec<String>, ApplyError> {
    let mut list = vec![];
    for service in services {
        match service.service_type {
            ServiceType::TCP {
                source,
                destination,
            } => list.push(generate_port_matcher("tcp", source, destination)),
            ServiceType::UDP {
                source,
                destination,
            } => list.push(generate_port_matcher("udp", source, destination)),
            // TODO Implement Packet type matching
            ServiceType::ICMP { ptypes: _ } => list.push("ip protocol icmp".to_string()),
            ServiceType::Group { .. } => (
                //TODO
            ),
        }
    }
    Ok(list)
}

fn generate_nat_action(
    address: Option<Address>,
    service: Option<Service>,
) -> Result<String, ApplyError> {
    let mut action;
    match address {
        Some(a) => {
            action = "ip to ".to_string()
                + &match a.address_type {
                    AddressType::Host { address } => address.to_string(),
                    _ => panic!("Invalid AddressType as Nat Action"),
                }
        }
        None => match service {
            Some(_) => action = "to ".to_string(),
            None => panic!("Address and Service can't both be None for Nat Action"),
        },
    }

    match service {
        Some(s) => match s.service_type {
            ServiceType::TCP { destination, .. } | ServiceType::UDP { destination, .. } => {
                match destination {
                    PortDefinition::Single { port } => {
                        action += ":";
                        action += &port.to_string()
                    }
                    _ => panic!("Destination Port Definition must be Single for Nat Action"),
                }
            }
            _ => panic!("ServiceType must be TCP or UDP for Nat Action"),
        },
        None => (),
    }
    Ok(action)
}

pub fn apply_nftables(pending_config: Config, _current_config: Config) -> Result<(), ApplyError> {
    let config_data;
    let mut context = Context::new();

    // Forward Rules
    let mut forward_rules = vec![];
    for rule in &pending_config.firewall.forward_rules {
        forward_rules.push(Rule {
            name: rule.name.clone(),
            counter: rule.counter,
            log: rule.log,
            addresses: generate_address_matcher(
                rule.source_addresses(pending_config.clone()),
                rule.negate_source,
                rule.destination_addresses(pending_config.clone()),
                rule.negate_destination,
            )?,
            services: generate_service_matchers(rule.services(pending_config.clone()))?,
            verdict: Some(match rule.verdict {
                Verdict::Accept => "accept".to_string(),
                Verdict::Drop => "drop".to_string(),
                Verdict::Continue => "continue".to_string(),
            }),
            destination_nat_action: None,
            source_nat_action: None,
        })
    }
    context.insert("forward_rules", &forward_rules);

    // Destination Nat Rules
    let mut destination_nat_rules = vec![];
    for rule in &pending_config.firewall.destination_nat_rules {
        destination_nat_rules.push(Rule {
            name: rule.name.clone(),
            counter: rule.counter,
            log: rule.log,
            addresses: generate_address_matcher(
                rule.source_addresses(pending_config.clone()),
                rule.negate_source,
                rule.destination_addresses(pending_config.clone()),
                rule.negate_destination,
            )?,
            services: generate_service_matchers(rule.services(pending_config.clone()))?,
            verdict: None,
            destination_nat_action: Some(
                "dnat ".to_string()
                    + &generate_nat_action(
                        rule.dnat_address(pending_config.clone()),
                        rule.dnat_service(pending_config.clone()),
                    )?,
            ),
            source_nat_action: None,
        })
    }
    context.insert("destination_nat_rules", &destination_nat_rules);

    // Source Nat Rules
    let mut source_nat_rules = vec![];
    for rule in &pending_config.firewall.source_nat_rules {
        source_nat_rules.push(Rule {
            name: rule.name.clone(),
            counter: rule.counter,
            log: rule.log,
            addresses: generate_address_matcher(
                rule.source_addresses(pending_config.clone()),
                rule.negate_source,
                rule.destination_addresses(pending_config.clone()),
                rule.negate_destination,
            )?,
            services: generate_service_matchers(rule.services(pending_config.clone()))?,
            verdict: None,
            destination_nat_action: None,
            source_nat_action: Some(match rule.snat_type.clone() {
                SNATType::Masquerade => "masquerade".to_string(),
                SNATType::SNAT { .. } => {
                    "snat ".to_string()
                        + &generate_nat_action(
                            rule.snat_type.address(pending_config.clone()),
                            rule.snat_type.service(pending_config.clone()),
                        )?
                }
            }),
        })
    }
    context.insert("source_nat_rules", &source_nat_rules);

    match templates::TEMPLATES.render(NFTABLES_TEMPLATE_PATH, &context) {
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

    info!("Deleting old nftables Config");
    std::fs::remove_file(NFTABLES_CONFIG_PATH)?;

    info!("Writing new nftables Config");
    let mut f = std::fs::File::create(NFTABLES_CONFIG_PATH)?;
    f.write_all(config_data.as_bytes())?;

    info!("Restarting nftables");
    match Command::new("systemctl")
        .arg("restart")
        .arg("nftables")
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
