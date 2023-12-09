use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate, Default, Debug)]
pub struct Firewall {
    pub forward_rules: Vec<ForwardRule>,
    pub destination_nat_rules: Vec<DestinationNATRule>,
    pub source_nat_rules: Vec<SourceNATRule>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct ForwardRule {
    pub name: String,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub comment: String,
    pub counter: bool,
    pub verdict: Verdict,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct DestinationNATRule {
    pub name: String,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub comment: String,
    pub counter: bool,
    pub dnat_address: Option<String>,
    pub dnat_service: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct SourceNATRule {
    pub name: String,
    pub services: Vec<String>,
    pub source_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
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
        address: Option<String>,
        service: Option<String>,
    },
    Masquerade,
}
