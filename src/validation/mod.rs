use {
    crate::definitions::config::Config, crate::definitions::vpn::Wireguard,
    garde::rules::pattern::Matcher, once_cell::sync::Lazy, regex::Regex,
};

const REGEX_NAME: &str = r"^[a-zA-Z0-9._-]*$";
const REGEX_WIREGUARD_KEY: &str = r"^[A-Za-z0-9+/]{42}[AEIMQUYcgkosw048]=$";
const REGEX_DEVICE: &str = r"^[A-Za-z0-9_.-]{1,15}$";

/// Ports reserved by the system that cannot be used by configurable services.
const RESERVED_PORTS: &[u64] = &[4444];

pub fn validate_name(value: &str, _: &Config) -> garde::Result {
    if value.len() > 32 {
        return Err(garde::Error::new("name is longer than 32"));
    }

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_NAME).unwrap());
    if !RE.is_match(value) {
        return Err(garde::Error::new("name must only contain a-zA-Z0-9._-"));
    }
    Ok(())
}

pub fn validate_wireguard_key(value: &str, _: &Config) -> garde::Result {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_WIREGUARD_KEY).unwrap());
    if !RE.is_match(value) {
        return Err(garde::Error::new("invalid wireguard key"));
    }
    Ok(())
}

pub fn validate_optional_wireguard_key(value: &Option<String>, ctx: &Config) -> garde::Result {
    match value.as_deref() {
        None | Some("") => Ok(()),
        Some(key) => validate_wireguard_key(key, ctx),
    }
}

/// A kernel interface name: at most 15 characters (IFNAMSIZ), no `/`, `:`
/// or whitespace, not `.` or `..` (the kernel's `dev_valid_name`), limited
/// further to a conservative character set.
pub fn validate_device(value: &str, _: &Config) -> garde::Result {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_DEVICE).unwrap());
    if !RE.is_match(value) || value == "." || value == ".." {
        return Err(garde::Error::new(
            "device must be 1-15 characters of a-zA-Z0-9_.- and not . or ..",
        ));
    }
    Ok(())
}

pub fn validate_wireguard_ports(value: &Wireguard, _: &Config) -> garde::Result {
    let mut seen = std::collections::BTreeSet::new();
    for iface in &value.interfaces {
        if RESERVED_PORTS.contains(&iface.listen_port) {
            return Err(garde::Error::new(format!(
                "listen port {} on interface '{}' is reserved for system use",
                iface.listen_port, iface.name
            )));
        }
        if !seen.insert(iface.listen_port) {
            return Err(garde::Error::new(format!(
                "duplicate listen port {} on interface '{}'",
                iface.listen_port, iface.name
            )));
        }
    }
    Ok(())
}

pub fn validate_endpoint_host(value: &str, ctx: &Config) -> garde::Result {
    let addr = ctx
        .object
        .addresses
        .iter()
        .find(|a| a.name == value)
        .ok_or_else(|| garde::Error::new(format!("address '{}' not found", value)))?;

    match &addr.address_type {
        crate::definitions::object::AddressType::Host { .. } => Ok(()),
        other => Err(garde::Error::new(format!(
            "endpoint '{}' is {:?}, must be a Host address",
            value, other
        ))),
    }
}

pub fn validate_firewall_uuids(
    conf: &crate::definitions::config::Config,
) -> Vec<structdb_core::ValidationError> {
    let mut errors = Vec::new();
    let mut uuids = std::collections::HashSet::new();

    for rule in &conf.firewall.forward_rules {
        if !uuids.insert(rule.uuid) {
            errors.push(structdb_core::ValidationError::DuplicateKey {
                collection: "firewall_rules".to_string(),
                item_type: "ForwardRule".to_string(),
                key: rule.uuid.to_string(),
            });
        }
    }
    for rule in &conf.firewall.destination_nat_rules {
        if !uuids.insert(rule.uuid) {
            errors.push(structdb_core::ValidationError::DuplicateKey {
                collection: "firewall_rules".to_string(),
                item_type: "DestinationNATRule".to_string(),
                key: rule.uuid.to_string(),
            });
        }
    }
    for rule in &conf.firewall.source_nat_rules {
        if !uuids.insert(rule.uuid) {
            errors.push(structdb_core::ValidationError::DuplicateKey {
                collection: "firewall_rules".to_string(),
                item_type: "SourceNATRule".to_string(),
                key: rule.uuid.to_string(),
            });
        }
    }
    for rule in &conf.firewall.inbound_rules {
        if !uuids.insert(rule.uuid) {
            errors.push(structdb_core::ValidationError::DuplicateKey {
                collection: "firewall_rules".to_string(),
                item_type: "InboundRule".to_string(),
                key: rule.uuid.to_string(),
            });
        }
    }

    errors
}

#[cfg(test)]
mod security_tests {
    use crate::definitions::config::Config;
    use garde::Validate;

    fn check(conf: &Config) -> Result<(), garde::Report> {
        conf.validate_with(conf)
    }

    const TEST_KEY: &str = "YNqHbfBQKaGvzefSSc7VVnSZNNrRqcOeUyBqahn0vF4=";

    fn create_wireguard_test_config() -> Config {
        use crate::definitions::object::{Address, AddressType};
        use crate::definitions::vpn::{WireguardEndpoint, WireguardInterface, WireguardPeer};
        let mut conf = Config::default();
        conf.object.addresses.push(Address {
            name: "host".to_string(),
            address_type: AddressType::Host {
                address: "192.0.2.1".parse().unwrap(),
            },
            comment: String::new(),
        });
        conf.object.addresses.push(Address {
            name: "net".to_string(),
            address_type: AddressType::Network {
                network: "192.0.2.0/24".parse().unwrap(),
            },
            comment: String::new(),
        });
        conf.vpn.wireguard.peers.push(WireguardPeer {
            name: "peer".to_string(),
            public_key: TEST_KEY.to_string(),
            preshared_key: Some(TEST_KEY.to_string()),
            allowed_ips: vec![],
            endpoint: WireguardEndpoint::Specify {
                address: "host".to_string(),
                port: 51820,
            },
            persistent_keepalive: None,
            comment: String::new(),
        });
        conf.vpn.wireguard.interfaces.push(WireguardInterface {
            name: "wg0".to_string(),
            public_key: TEST_KEY.to_string(),
            private_key: TEST_KEY.to_string(),
            listen_port: 51820,
            peers: vec!["peer".to_string()],
            comment: String::new(),
        });
        conf
    }

    #[test]
    fn wireguard_keys_and_endpoints() {
        use crate::definitions::vpn::WireguardEndpoint;
        let conf = create_wireguard_test_config();
        assert!(check(&conf).is_ok());

        let mut bad = conf.clone();
        bad.vpn.wireguard.interfaces[0].private_key = format!("{}\nPostUp=x", TEST_KEY);
        assert!(check(&bad).is_err());

        let mut bad = conf.clone();
        bad.vpn.wireguard.peers[0].public_key = "short=".to_string();
        assert!(check(&bad).is_err());

        let mut bad = conf.clone();
        bad.vpn.wireguard.peers[0].preshared_key = Some("x\n[NetDev]".to_string());
        assert!(check(&bad).is_err());

        let mut ok = conf.clone();
        ok.vpn.wireguard.peers[0].preshared_key = None;
        assert!(check(&ok).is_ok());

        let mut bad = conf.clone();
        bad.vpn.wireguard.peers[0].endpoint = WireguardEndpoint::Specify {
            address: "net".to_string(),
            port: 51820,
        };
        assert!(check(&bad).is_err());

        let mut bad = conf.clone();
        bad.vpn.wireguard.peers[0].endpoint = WireguardEndpoint::Specify {
            address: "missing".to_string(),
            port: 51820,
        };
        assert!(check(&bad).is_err());
    }

    #[test]
    fn devices_and_vlan_ids() {
        use crate::definitions::network::{
            AddressingMode, NetworkInterface, NetworkInterfaceType, Vlan,
        };
        let mut conf = Config::default();
        conf.network.interfaces.push(NetworkInterface {
            name: "lan".to_string(),
            alias: String::new(),
            comment: String::new(),
            interface_type: NetworkInterfaceType::Hardware {
                device: "eth0".to_string(),
            },
            addressing_mode: AddressingMode::None,
            virtual_router: None,
        });
        conf.network.interfaces.push(NetworkInterface {
            name: "vlan10".to_string(),
            alias: String::new(),
            comment: String::new(),
            interface_type: NetworkInterfaceType::Vlan(Vlan {
                id: 10,
                parent: "lan".to_string(),
            }),
            addressing_mode: AddressingMode::None,
            virtual_router: None,
        });
        assert!(check(&conf).is_ok());

        for device in [
            "",
            "eth0\n[Match]",
            "averyveryverylongname",
            "eth0:1",
            ".",
            "..",
        ] {
            let mut bad = conf.clone();
            bad.network.interfaces[0].interface_type = NetworkInterfaceType::Hardware {
                device: device.to_string(),
            };
            assert!(check(&bad).is_err(), "{:?}", device);
        }

        for id in [0, 4095, -1] {
            let mut bad = conf.clone();
            bad.network.interfaces[1].interface_type = NetworkInterfaceType::Vlan(Vlan {
                id,
                parent: "lan".to_string(),
            });
            assert!(check(&bad).is_err(), "{}", id);
        }
    }
}
