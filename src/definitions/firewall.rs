use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug)]
pub struct Firewall {
    forward_rules: Vec<ForwardRule>,
    destination_nat_rules: Vec<DestinationNATRule>,
    source_nat_rules: Vec<SourceNATRule>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ForwardRule {
    pub name: String,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub comment: String,
    pub counter: bool,
    pub verdict: Verdict,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct DestinationNATRule {
    pub name: String,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub comment: String,
    pub counter: bool,
    pub dnat_address: String,
    pub dnat_service: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SourceNATRule {
    pub name: String,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub comment: String,
    pub counter: bool,
    pub snat_type: SNATType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Accept,
    Drop,
    Continue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SNATType {
    SNAT {
        snat_address: String,
        snat_service: String,
    },
    Masquerade,
}
