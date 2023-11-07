use serde::{Deserialize, Serialize};
use validator::Validate;

use super::object::{AddressReference, ServiceReference};

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Firewall {
    pub forward_rules: Vec<ForwardRule>,
    pub destination_nat_rules: Vec<DestinationNATRule>,
    pub source_nat_rules: Vec<SourceNATRule>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct ForwardRule {
    pub name: String,
    pub services: Vec<ServiceReference>,
    pub source_addresses: Vec<AddressReference>,
    pub destination_addresses: Vec<AddressReference>,
    pub comment: String,
    pub counter: bool,
    pub verdict: Verdict,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct DestinationNATRule {
    pub name: String,
    pub services: Vec<ServiceReference>,
    pub source_addresses: Vec<AddressReference>,
    pub destination_addresses: Vec<AddressReference>,
    pub comment: String,
    pub counter: bool,
    pub dnat_address: Option<AddressReference>,
    pub dnat_service: Option<ServiceReference>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct SourceNATRule {
    pub name: String,
    pub services: Vec<ServiceReference>,
    pub source_addresses: Vec<AddressReference>,
    pub destination_addresses: Vec<AddressReference>,
    pub comment: String,
    pub counter: bool,
    pub snat_type: SNATType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Accept,
    Drop,
    Continue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SNATType {
    SNAT {
        address: Option<AddressReference>,
        service: Option<ServiceReference>,
    },
    Masquerade,
}
