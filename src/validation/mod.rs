use {
    crate::definitions::config::Config, crate::definitions::vpn::Wireguard,
    garde::rules::pattern::Matcher, once_cell::sync::Lazy, regex::Regex,
};

const REGEX_NAME: &str = r"^[a-zA-Z0-9._/-]*$";

/// Ports reserved by the system that cannot be used by configurable services.
const RESERVED_PORTS: &[u64] = &[4444];

pub fn validate_name(value: &str, _: &Config) -> garde::Result {
    if value.len() > 32 {
        return Err(garde::Error::new("name is longer than 32"));
    }

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_NAME).unwrap());
    if !RE.is_match(value) {
        return Err(garde::Error::new("name must only contain a-zA-Z0-9._/-"));
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
